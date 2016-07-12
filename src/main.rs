extern crate akari_solver;

use std::io;
use std::io::Read;
use std::fs::File;
use std::process::Command;
use std::env;

use akari_solver::grid as solver;
use akari_solver::grid::utils;
use akari_solver::grid::rules;
use akari_solver::grid::cnf_format;

static CNF_OUT: &'static str = "/tmp/akari-solver-cnf-out.cnf";
static RESULT_IN: &'static str = "/tmp/akari-solver-result-in.cnf";
static SAT_SOLVER_ENV_NAME: &'static str = "SAT_SOLVER";

fn main() {
    let grid_str = read_grid_string().unwrap();
    let dim = (grid_str.len() as f64).sqrt() as i32;
    if dim * dim != grid_str.replace(char::is_whitespace, "").len() as i32 {
        println!("Error: Grid is non-square.");
        return;
    }
    let mut grid = utils::precompute_data(utils::get_grid_from_string(&grid_str, dim).unwrap());
    let (_, is_uniq) = solver::solve_puzzle(&mut grid, solve_sat_with_glucose).unwrap();
    println!("{}", utils::print_griddata_to_string(&grid, true));
    println!("Unique solution: {}", is_uniq);
}

fn read_int_line() -> Result<i32, String> {
    let mut line = String::new();

    try!(io::stdin().read_line(&mut line).map_err(|e| e.to_string()));
    let res = try!(line.trim().parse::<i32>().map_err(|e| e.to_string()));
    Ok(res)
}

fn read_grid_string() -> Result<String, String> {
    let mut grid_raw = String::new();
    try!(io::stdin().read_to_string(&mut grid_raw).map_err(|e| e.to_string()));
    Ok(grid_raw)
}

fn solve_sat_with_glucose(cnf: &cnf_format::CnfFormula) -> Result<Vec<i32>, String> {
    {
        let mut cnf_file = try!(File::create(CNF_OUT)
                                .map_err(|e| format!("Error creating CNF file: {}",
                                                     e.to_string())));
        try!(cnf.write_to_file(&mut cnf_file)
             .map_err(|e| format!("Error writing to file: {}", e.to_string())));
    }
    let sat_solver_path =
        try!(env::var(SAT_SOLVER_ENV_NAME)
             .map_err(|x| format!("Error finding SAT solver: {:?}", x)));

    let sat_result = Command::new(sat_solver_path)
        .arg(CNF_OUT)
        .arg(RESULT_IN)
        .output();

    try!(sat_result.map_err(|e| format!("Error executing SAT solver: {}", e.to_string())));
    read_variable_mapping(RESULT_IN)
}

fn read_variable_mapping(filepath: &str) -> Result<Vec<i32>, String> {
    let mut infile = try!(File::open(filepath).map_err(|e| e.to_string()));
    let mut result_string = String::new();
    let mut result = Vec::new();
    try!(infile.read_to_string(&mut result_string).map_err(|e| e.to_string()));
    for var_str in result_string.split_whitespace() {
        if let Ok(v) = var_str.trim().parse::<i32>() {
            if v != 0 {
                result.push(v);
            }
        } else {
            return Err(format!("Encountered {} and had error", var_str.trim()));
        };
    }
    Ok(result)
}
