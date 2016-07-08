use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED, INVALID_POSITION};
use super::{Grid, GridData};
use super::utils::get_neighbors;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec::Vec;

const CANNOT_BE_LIGHT: u8 = IS_SOLID | IS_LIT | IS_LIGHT | CANT_LIGHT;
const CONSTRAINT_NUM_MASK: u8 = 0x7;

pub struct CnfFormula {
    grid_to_cnf_position_mapping: HashMap<usize, i32>,
    cnf_to_grid_position_mapping: HashMap<i32, usize>,
    clauses: Vec<Vec<i32>>
}

struct ConstraintCnfGenerator {
    constraint_size: u32,
    cnf_clauses: Vec<(u32, Vec<bool>)>
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

pub fn make_cnf_formula(grid: &GridData) { //-> CnfFormula {
    let (grid_to_cnf, cnf_to_grid) = produce_variable_mapping(grid);
    //let mut clauses = Vec::new();
    let mut sorted_cnf_ids = cnf_to_grid.keys().cloned().collect::<Vec<i32>>();
    sorted_cnf_ids.sort();
    for grid_idx in 0..(grid.grid.size * grid.grid.size) {
        if can_disregard(grid.grid.contents[grid_idx as usize]) {
            continue;
        }
        //let primary_clause = if grid.grid.contents[grid_idx] & IS_CONSTRAINED != 0 {
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

fn get_numerical_constraint_clauses(grid: &GridData,
                                   to_cnf: HashMap<usize, i32>, loc: usize) -> Vec<Vec<i32>> {
    let adj_neighbors = get_neighbors(grid, loc).1[..4]
        .into_iter()
        .filter(|&&x| x != INVALID_POSITION)
        .map(usize::clone)
        .collect::<Vec<usize>>();

    let num_existing_lights = adj_neighbors.iter()
        .fold(0, |a, &x| if grid.grid.contents[x] & IS_LIT != 0 { a + 1 } else { a });
    let constraint_num = grid.grid.contents[loc] & CONSTRAINT_NUM_MASK - num_existing_lights;
    let possible_satisfying_cnf_ids = adj_neighbors
        .into_iter()
        .filter_map(|x| to_cnf.get(&x))
        .map(i32::clone)
        .collect::<Vec<i32>>();
}
