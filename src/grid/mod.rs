use std::collections::HashMap;
use std::usize;

pub mod utils;
pub mod rules;
pub mod cnf_format;

const IS_SOLID: u8 = 1<<3;
const IS_LIT: u8 = 1<<4;
const IS_LIGHT: u8 = 1<<5;
const CANT_LIGHT: u8 = 1<<6;
const IS_CONSTRAINED: u8 = 1<<7;

const INVALID_POSITION: usize = usize::MAX;

pub struct Grid {
    contents: Vec<u8>,
    size: i32 
}

pub struct GridData {
    grid: Grid,
    sight_lines: HashMap<usize, Vec<usize>>
}

pub type SatSolver = fn(&cnf_format::CnfFormula) -> Result<Vec<i32>, String>;

/**
 * Solves the given puzzle, possibly mutating it in the process.
 * Returns a pair, where the first entry is a Vec of light locations,
 * and the second denotes whether the solution is unique.
 */
pub fn solve_puzzle(grid: &mut GridData, solver: SatSolver) -> Result<(Vec<usize>, bool), String> {
    rules::populate_with_rules(grid);
    let cnf_formula = cnf_format::make_cnf_formula(&grid);
    let solver_result = try!(solver(&cnf_formula));
    let cnf_formula_excluding_previous_soln =
        cnf_formula.append_inverse(&solver_result);

    let is_unique = match solver(&cnf_formula_excluding_previous_soln) {
        Ok(_) => false,
        Err(_) => true
    };
    cnf_format::populate_grid_with_cnf(grid, &cnf_formula, solver_result);

    let lit_locs = (0..((grid.grid.size * grid.grid.size) as usize))
        .filter(|&x| (grid.grid.contents[x] & IS_LIGHT) != 0)
        .collect();
    Ok((lit_locs, is_unique))
}
