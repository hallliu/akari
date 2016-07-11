use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED, INVALID_POSITION};
use super::{Grid, GridData};
use super::utils::get_neighbors;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec::Vec;
use std::iter::Map;
use std::boxed::Box;
use std::io::{Write, Result};

#[cfg(test)]
mod tests;

const CANNOT_BE_LIGHT: u8 = IS_SOLID | IS_LIT | IS_LIGHT | CANT_LIGHT;
const CONSTRAINT_NUM_MASK: u8 = 0x7;

pub struct CnfFormula {
    grid_to_cnf_position_mapping: HashMap<usize, i32>,
    cnf_to_grid_position_mapping: HashMap<i32, usize>,
    clauses: Vec<Vec<i32>>
}

impl CnfFormula {
    pub fn write_to_file<T>(&self, file: &mut T) -> Result<()> where T: Write {
        try!(write!(file, "p cnf {} {}\n",
                    self.grid_to_cnf_position_mapping.len(),
                    self.clauses.len()));
        for clause in self.clauses.iter() {
            for &num in clause.iter() {
                try!(write!(file, "{} ", num));
            }
            try!(write!(file, "0\n"));
        }
        Ok(())
    }
}

struct ConstraintCnfGenerator {
    total_size: u32,
    cnf_clauses: Vec<Vec<(u32, Vec<bool>)>>
}

fn make_constraint_cnf_generator(size: u32) -> ConstraintCnfGenerator {
    let clauses = (0..(size + 1)).map(|i| make_constraint_cnf_cache(i)).collect();
    ConstraintCnfGenerator { total_size: size, cnf_clauses: clauses }
}

fn make_constraint_cnf_cache(size: u32) -> Vec<(u32, Vec<bool>)> { 
    let mut to_explore: VecDeque<(u32, Vec<bool>)> = VecDeque::new();
    let mut result: Vec<(u32, Vec<bool>)> = Vec::new();
    to_explore.push_front((0, Vec::new()));
    loop {
        let mut next = match to_explore.pop_back() {
            Some(x) => x,
            None => { break; }
        };
        if next.1.len() as u32 == size {
            result.push(next);
        } else {
            let mut t_branch = next.1.to_vec();
            t_branch.push(true);
            to_explore.push_front((next.0 + 1, t_branch));
            next.1.push(false);
            to_explore.push_front((next.0, next.1));
        }
    }
    result
}

impl ConstraintCnfGenerator {
    fn get_constraints<'a>(&'a self, sat_ids: Vec<i32>, num_true: u32) -> Box<Iterator<Item=Vec<i32>> + 'a> {
        let ref bool_arrays = self.cnf_clauses[sat_ids.len()];
        Box::new(bool_arrays.iter()
            .filter(move |&x| x.0 != num_true)
            .map(move |x| x.1.iter().enumerate().map(|y: (usize, &bool)| {
                if *y.1 {
                    -sat_ids[y.0]
                } else {
                    sat_ids[y.0]
                }
            }).collect()))
    }
}

pub fn make_cnf_formula(grid: &GridData) -> CnfFormula {
    let (grid_to_cnf, cnf_to_grid) = produce_variable_mapping(grid);
    let mut clauses = Vec::new();
    let mut sorted_cnf_ids = cnf_to_grid.keys().cloned().collect::<Vec<i32>>();
    let constraint_cnf_gen = make_constraint_cnf_generator(4);

    sorted_cnf_ids.sort();
    for grid_idx in 0..((grid.grid.size * grid.grid.size) as usize) {
        if can_disregard(grid.grid.contents[grid_idx]) {
            continue;
        }
        if grid.grid.contents[grid_idx] & IS_CONSTRAINED != 0 {
            clauses.extend(get_numerical_constraint_clauses(grid, &constraint_cnf_gen, &grid_to_cnf, grid_idx));
        } 
        if does_need_light(grid.grid.contents[grid_idx]) {
            clauses.push(get_sight_line_clauses(&grid, &grid_to_cnf, grid_idx));
        }
    }

    let cnf_ids_within_sight = get_cnf_ids_within_sight(grid, &cnf_to_grid, &grid_to_cnf);
    clauses.extend(
        cnf_ids_within_sight.into_iter()
        .map(|x| vec![-x.0, -x.1]));

    CnfFormula {
        grid_to_cnf_position_mapping: grid_to_cnf,
        cnf_to_grid_position_mapping: cnf_to_grid,
        clauses: clauses
    }
}

fn produce_variable_mapping(grid: &GridData) -> (HashMap<usize, i32>, HashMap<i32, usize>) {
    let mut result_map = HashMap::new();
    let mut reverse_map = HashMap::new();
    let mut cnf_idx = 1i32;
    for (idx, val) in grid.grid.contents.iter().enumerate() {
        if val & CANNOT_BE_LIGHT == 0 {
            result_map.insert(idx, cnf_idx);
            reverse_map.insert(cnf_idx, idx);
            cnf_idx += 1;
        }
    }
    (result_map, reverse_map)
}

fn does_need_light(val: u8) -> bool {
    val & IS_SOLID == 0
        && val & IS_LIT == 0
        && val & IS_LIGHT == 0
}

fn can_disregard(val: u8) -> bool {
    (val & IS_SOLID != 0 && val & IS_CONSTRAINED == 0)
        || (val & IS_SOLID == 0 && (val & IS_LIT != 0 || val & IS_LIGHT != 0))
}

fn get_cnf_ids_within_sight(grid: &GridData, cnf_to_grid: &HashMap<i32, usize>,
                            grid_to_cnf: &HashMap<usize, i32>) -> HashSet<(i32, i32)> {
    let mut cnf_ids_within_sight: HashSet<(i32, i32)> = HashSet::new();
    for (cnf_idx, grid_idx) in cnf_to_grid {
        let ref sight_line = match grid.sight_lines.get(grid_idx) {
            Some(x) => x,
            None => { continue; }
        };
        cnf_ids_within_sight.extend(
            sight_line.iter().filter_map(|x| grid_to_cnf.get(x))
            .filter(|&x| x > cnf_idx)
            .cloned()
            .map(|x| (*cnf_idx, x)));
    }
    cnf_ids_within_sight
}

fn get_sight_line_clauses(grid: &GridData, to_cnf: &HashMap<usize, i32>, loc: usize) -> Vec<i32> {
    let mut result = Vec::new();
    match to_cnf.get(&loc) {
        Some(i) => { result.push(*i); },
        None => { }
    };

    let ref sight_line = match grid.sight_lines.get(&loc) {
        Some(x) => x,
        None => { return result; }
    };
    result.extend(sight_line.iter().filter_map(|x| to_cnf.get(x).cloned()));
    result
}

fn get_numerical_constraint_clauses<'a, 'b>(
    grid: &'a GridData, gen: &'b ConstraintCnfGenerator,
    to_cnf: &'a HashMap<usize, i32>, loc: usize) -> Box<Iterator<Item=Vec<i32>> + 'b> {
    let adj_neighbors = get_neighbors(grid, loc).1[..4]
        .into_iter()
        .filter(|&&x| x != INVALID_POSITION)
        .map(usize::clone)
        .collect::<Vec<usize>>();

    let num_existing_lights = adj_neighbors.iter()
        .fold(0, |a, &x| if grid.grid.contents[x] & IS_LIGHT != 0 { a + 1 } else { a });
    let constraint_num = (grid.grid.contents[loc] & CONSTRAINT_NUM_MASK) - num_existing_lights;
    let possible_satisfying_cnf_ids = adj_neighbors
        .into_iter()
        .filter_map(|x| to_cnf.get(&x))
        .map(i32::clone)
        .collect::<Vec<i32>>();
    gen.get_constraints(possible_satisfying_cnf_ids, constraint_num as u32)
}
