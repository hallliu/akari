extern crate akari_solver;

use std::io;
use std::io::Read;
use std::fs::File;
use std::process::Command;

use akari_solver::grid::utils;
use akari_solver::grid::rules;
use akari_solver::grid::cnf_format;

static CNF_OUT: &'static str = "/tmp/akari-solver-tmp-out.cnf";
static RESULT_IN: &'static str = "/tmp/akari-solver-tmp-in.cnf";

fn main() {
    let n4 = "-2".parse::<i32>().unwrap();
    let grid_str = read_grid_string().unwrap();
    let dim = (grid_str.len() as f64).sqrt() as i32;
    if dim * dim != grid_str.replace(char::is_whitespace, "").len() as i32 {
        println!("Error: Grid is non-square.");
        return;
    }
    let mut grid = utils::precompute_data(utils::get_grid_from_string(&grid_str, dim).unwrap());
    rules::populate_with_rules(&mut grid);

    let cnf_formula = cnf_format::make_cnf_formula(&grid);
    {
        let mut cnf_file = File::create(CNF_OUT).unwrap();
        cnf_formula.write_to_file(&mut cnf_file);
    }
    Command::new("./glucose")
        .arg(CNF_OUT)
        .arg(RESULT_IN)
        .output().expect("Failed to execute SAT solver");
    let cnf_light_locations = read_variable_mapping(RESULT_IN).unwrap();
    cnf_format::populate_grid_with_cnf(&mut grid, &cnf_formula, cnf_light_locations);
    println!("{}", utils::print_griddata_to_string(&grid, true));
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

fn read_variable_mapping(filepath: &str) -> Result<Vec<i32>, String> {
    let mut infile = try!(File::open(filepath).map_err(|e| e.to_string()));
    let mut result_string = String::new();
    let mut result = Vec::new();
    try!(infile.read_to_string(&mut result_string).map_err(|e| e.to_string()));
    for var_str in result_string.split_whitespace() {
        if let Ok(v) = var_str.trim().parse::<i32>() {
            if v > 0 {
                result.push(v);
            }
        } else {
            return Err(format!("Encountered {} and had error", var_str.trim()));
        };
    }
    Ok(result)
}
