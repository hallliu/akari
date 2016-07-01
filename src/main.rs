extern crate akari_solver;

use std::io;
use std::io::Read;
use akari_solver::grid::utils;

fn main() {
    loop {
        let dim = match read_int_line() {
            Ok(n) => n,
            Err(msg) => {
                println!("{}", msg);
                break;
            }
        };
        if dim <= 0 {
            break;
        }

        
        let grid_str = read_grid_string().unwrap();
        let grid = utils::precompute_data(utils::get_grid_from_string(&grid_str, dim).unwrap());
    }
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
