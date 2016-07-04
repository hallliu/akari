use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED};
use super::*;
use std::collections::HashMap;
use std::char;

#[cfg(test)]
mod tests;

// The numbering scheme for neighbors and corners looks like this:
// 4 0 5
// 3 X 1
// 7 2 6
// where X is the location in question.

#[allow(unused_variables)]
static EDGE_SPECIFIERS: [fn(usize, usize) -> bool; 4] = [
    { fn t(loc: usize, size: usize) -> bool { loc < size }; t},
    { fn t(loc: usize, size: usize) -> bool { loc % size == 0}; t}, 
    { fn t(loc: usize, size: usize) -> bool { loc + size >= size * size }; t},
    { fn t(loc: usize, size: usize) -> bool { loc % size == size - 1 }; t}
];

static EDGE_INVALIDATED_POSITIONS: [[usize; 3]; 4] = [
    [4, 0, 5],
    [4, 3, 7],
    [7, 2, 6],
    [5, 1, 6]
];

#[allow(unused_variables)]
static RELATIVE_POSITION_SPECIFIERS: [fn(usize, usize) -> usize; 8] = [
    { fn t(loc: usize, size: usize) -> usize { loc - size }; t},
    { fn t(loc: usize, size: usize) -> usize { loc + 1 }; t},
    { fn t(loc: usize, size: usize) -> usize { loc + size }; t},
    { fn t(loc: usize, size: usize) -> usize { loc - 1 }; t},
    { fn t(loc: usize, size: usize) -> usize { loc - size - 1 }; t},
    { fn t(loc: usize, size: usize) -> usize { loc - size + 1 }; t},
    { fn t(loc: usize, size: usize) -> usize { loc + size + 1 }; t},
    { fn t(loc: usize, size: usize) -> usize { loc + size - 1 }; t},
];

pub fn get_neighbors(grid: &GridData, loc: usize) -> ([bool; 8], [usize; 8]) {
    let grid_size = grid.grid.size as usize;
    let contents = &grid.grid.contents;

    let mut res = [super::INVALID_POSITION; 8];
    let mut valid = [true; 8];
    let mut is_empty = [false; 8];
    let cannot_light = IS_SOLID | IS_LIT | CANT_LIGHT | IS_LIGHT;

    for (spec, inv_pos) in EDGE_SPECIFIERS.iter().zip(EDGE_INVALIDATED_POSITIONS.iter()) {
        if spec(loc, grid_size) {
            for &i in inv_pos.iter() {
                valid[i] = false;
            }
        }
    }

    for (idx, pos_spec) in RELATIVE_POSITION_SPECIFIERS.iter().enumerate() {
        if valid[idx] {
            let nbr_pos = pos_spec(loc, grid_size);
            res[idx] = nbr_pos;
            is_empty[idx] = contents[nbr_pos] & cannot_light == 0;
        }
    }
    (is_empty, res)
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
    for i in 0..(grid.size * grid.size) {
        if grid.contents[i as usize] & IS_SOLID == 0 {
            sight_lines.insert(i as usize, get_sight_line(&grid, i));
        }
    }
    GridData {grid: grid, sight_lines: sight_lines}
}

pub fn get_sight_line(grid: &Grid, idx: i32) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity((grid.size * 2 - 1) as usize);
    
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

    result.sort();
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
                b'^' => CANT_LIGHT,
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
        } else if val & CANT_LIGHT != 0 {
            s.push('^');
        } else {
            s.push('_');
        }
    }
    s
}
