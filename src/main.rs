extern crate akari_solver;
extern crate getopts;

use std::io;
use std::io::Read;
use std::fs::File;
use std::process::Command;
use std::env;

use getopts::Options;

use akari_solver::grid as solver;
use akari_solver::grid::utils;
use akari_solver::grid::cnf_format;

static CNF_OUT: &'static str = "/tmp/akari-solver-cnf-out.cnf";
static RESULT_IN: &'static str = "/tmp/akari-solver-result-in.cnf";
static SAT_SOLVER_ENV_NAME: &'static str = "SAT_SOLVER";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();

    options.optflag("p", "pretty-print", "Pretty-print the solution.");
    options.optflag("u", "unique-only", "Only print whether the solution is unique.");
    options.optflag("h", "help", "Print the usage");
    let matches = match options.parse(&args[1..]) {
        Ok(x) => x,
        Err(y) => { panic!(y.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&args[0], &options);
        return;
    }

    let pretty_print = matches.opt_present("p");
    let unique_only = matches.opt_present("u");

    let (height, width) = read_grid_dims().unwrap();
    let grid_str = read_grid_string().unwrap();

    let mut grid = utils::precompute_data(utils::get_grid_from_string(&grid_str, height, width).unwrap());
    let (light_locs, is_uniq) = solver::solve_puzzle(&mut grid, solve_sat_with_glucose).unwrap();
    if pretty_print {
        println!("{}", utils::print_griddata_to_string(&grid, true));
        println!("Unique solution: {}", is_uniq);
    }
    else {
        if !unique_only {
            for loc in light_locs {
                print!("{} ", loc);
            }
            println!("");
        }
        println!("{}", if is_uniq { 1 } else { 0 });
    }
}

fn print_usage(progname: &str, opts: &Options) {
    let desc = format!("\
    Usage: {} [-p|--pretty-print]

    Takes a puzzle to solve from standard input, solves it, and outputs the solution.
    Input format: First line consists of two numbers separated by a space,
    corresponding to the height and width of the puzzle, respectively.
    Subsequent lines are interpreted as a string, where each non-whitespace character
    corresponds to a square on the grid in row-major order. The meaning of characters
    is as follows:
    X -- Solid block
    _ -- Empty block
    0, 1, 2, 3, 4 -- Solid blocks that carry a surrounding lights constraint

    Output will be produced on standard out.
    If --pretty-print is not specified, output will be a list of indices that contain
    lights, again in row-major order, followed by a newline, followed by 1 if the
    solution is unique, and 0 otherwise.

    If --pretty print is specified, output will be a formatted grid that contains a
    solution, formatted the same, but with the following possible characters:
    * -- Square that contains a light
    # -- Square that has been lit\
    ", progname);

    print!("{}", opts.usage(&desc));
}

fn read_grid_dims() -> Result<(i32, i32), String> {
    let mut input_line = String::new();
    try!(io::stdin().read_line(&mut input_line).map_err(|e| e.to_string()));
    let maybe_dims = input_line.split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();
    let dims = try!(maybe_dims.map_err(|e| e.to_string()));
    if dims.len() != 2 {
        return Err(format!("Found {} dimensions instead of 2.", dims.len()));
    }
    Ok((dims[0], dims[1]))
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
