use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED};
use super::*;
use std::collections::HashMap;
use std::char;

mod tests;

pub fn get_adj_empties(grid: &GridData, loc: usize) -> ([bool; 4], [usize; 4]) {
    let grid_size = grid.grid.size as usize;
    let mut res = [super::INVALID_POSITION; 4];
    let mut valid = [false; 4];
    let cannot_light = IS_SOLID | IS_LIT | CANT_LIGHT | IS_LIGHT;

    if loc >= grid_size {
        res[0] = loc - grid_size;
        valid[0] = grid.grid.contents[res[0]] & cannot_light == 0;
    }
    if loc % grid_size != grid_size - 1 {
        res[1] = loc + 1;
        valid[1] = grid.grid.contents[res[1]] & cannot_light == 0;
    }
    if loc + grid_size < grid_size * grid_size {
        res[2] = loc + grid_size;
        valid[2] = grid.grid.contents[res[2]] & cannot_light == 0;
    }
    if loc % grid_size != 0 {
        res[3] = loc - 1;
        valid[3] = grid.grid.contents[res[3]] & cannot_light == 0;
    }
    (valid, res)
}

pub fn insert_light(grid: &mut GridData, loc: usize) {
    let cannot_light = IS_SOLID | IS_LIT | CANT_LIGHT | IS_LIGHT;
    if grid.grid.contents[loc] & cannot_light != 0 {
        return;
    }
    match grid.sight_lines.get(&loc) {
        Some(sl) => for lit_loc in sl.iter() {
            grid.grid.contents[*lit_loc] |= IS_LIT;
        },
        None => {return;}
    }
    grid.grid.contents[loc] |= IS_LIGHT | IS_LIT;
}

pub fn precompute_data(grid: Grid) -> GridData {
    let mut sight_lines = HashMap::new();
    for i in 0..(grid.size * grid.size - 1) {
        if grid.contents[i as usize] & IS_SOLID == 0 {
            sight_lines.insert(i as usize, get_sight_line(&grid, i));
        }
    }
    GridData {grid: grid, sight_lines: sight_lines}
}

pub fn get_sight_line(grid: &Grid, idx: i32) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity((grid.size * 2 - 1) as usize);
    result.push(idx as usize);
    
    let mut right_idx = idx + 1;
    while right_idx % grid.size != 0 {
        if grid.contents[right_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(right_idx as usize);
        right_idx += 1;
    }

    let mut left_idx = idx - 1;
    while left_idx % grid.size != grid.size - 1 && left_idx >= 0 {
        if grid.contents[left_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(left_idx as usize);
        left_idx -= 1;
    }

    let mut up_idx = idx - grid.size;
    while up_idx >= 0 {
        if grid.contents[up_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(up_idx as usize);
        up_idx -= grid.size;
    }

    let mut down_idx = idx + grid.size;
    while down_idx < grid.size * grid.size {
        if grid.contents[down_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(down_idx as usize);
        down_idx += grid.size;
    }

    result
}

pub fn get_grid_from_string(input: &str, num_squares: i32) -> Result<Grid, String> {
    let mut data: Vec<u8> = Vec::with_capacity((num_squares * num_squares) as usize);
    for grid_line in input.split_whitespace() {
        if grid_line.as_bytes().len() != num_squares as usize {
            return Err(From::from("Input length mismatch"));
        }
        for c in grid_line.as_bytes() {
            data.push(match *c {
                b'X' => IS_SOLID,
                b'0' => IS_SOLID | IS_CONSTRAINED,
                b'1' => 1 | IS_SOLID | IS_CONSTRAINED,
                b'2' => 2 | IS_SOLID | IS_CONSTRAINED,
                b'3' => 3 | IS_SOLID | IS_CONSTRAINED,
                b'4' => 4 | IS_SOLID | IS_CONSTRAINED,
                _ => 0
            });
        }
    }
    Ok(Grid {contents: data, size: num_squares})
}

pub fn print_grid_to_string(grid: &Grid, pretty_print: bool) -> String {
    let dim = (grid.size * grid.size) as usize;
    let mut s = String::with_capacity(dim);
    for idx in 0..dim {
        let val = grid.contents[idx];
        if pretty_print && ((idx as i32) % grid.size == 0) {
            s.push('\n');
        }
        if val & IS_LIGHT != 0 {
            s.push('*');
        } else if val & IS_CONSTRAINED != 0 {
            s.push(char::from_digit((val & 0x7) as u32, 10).expect(""));
        } else if val & IS_SOLID != 0 {
            s.push('X');
        } else if val & IS_LIT != 0 {
            s.push('#');
        } else {
            s.push('_');
        }
    }
    s
}
