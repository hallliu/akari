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
static EDGE_SPECIFIERS: [fn(usize, usize, usize) -> bool; 4] = [
    { fn t(loc: usize, height: usize, width: usize) -> bool { loc < width }; t},
    { fn t(loc: usize, height: usize, width: usize) -> bool { loc % width == 0}; t}, 
    { fn t(loc: usize, height: usize, width: usize) -> bool { loc + width >= height * width }; t},
    { fn t(loc: usize, height: usize, width: usize) -> bool { loc % width == width - 1 }; t}
];

static EDGE_INVALIDATED_POSITIONS: [[usize; 3]; 4] = [
    [4, 0, 5],
    [4, 3, 7],
    [7, 2, 6],
    [5, 1, 6]
];

#[allow(unused_variables)]
static RELATIVE_POSITION_SPECIFIERS: [fn(usize, usize) -> usize; 8] = [
    { fn t(loc: usize, width: usize) -> usize { loc - width }; t},
    { fn t(loc: usize, width: usize) -> usize { loc + 1 }; t},
    { fn t(loc: usize, width: usize) -> usize { loc + width }; t},
    { fn t(loc: usize, width: usize) -> usize { loc - 1 }; t},
    { fn t(loc: usize, width: usize) -> usize { loc - width - 1 }; t},
    { fn t(loc: usize, width: usize) -> usize { loc - width + 1 }; t},
    { fn t(loc: usize, width: usize) -> usize { loc + width + 1 }; t},
    { fn t(loc: usize, width: usize) -> usize { loc + width - 1 }; t},
];

pub fn get_neighbors(grid: &GridData, loc: usize) -> ([bool; 8], [usize; 8]) {
    let contents = &grid.grid.contents;
    let (height, width) = (grid.grid.height as usize, grid.grid.width as usize);

    let mut res = [super::INVALID_POSITION; 8];
    let mut valid = [true; 8];
    let mut is_empty = [false; 8];
    let cannot_light = IS_SOLID | IS_LIT | CANT_LIGHT | IS_LIGHT;

    for (spec, inv_pos) in EDGE_SPECIFIERS.iter().zip(EDGE_INVALIDATED_POSITIONS.iter()) {
        if spec(loc, height, width) {
            for &i in inv_pos.iter() {
                valid[i] = false;
            }
        }
    }

    for (idx, pos_spec) in RELATIVE_POSITION_SPECIFIERS.iter().enumerate() {
        if valid[idx] {
            let nbr_pos = pos_spec(loc, width);
            res[idx] = nbr_pos;
            is_empty[idx] = contents[nbr_pos] & cannot_light == 0;
        }
    }
    (is_empty, res)
}

pub fn insert_light(grid: &mut GridData, loc: usize) -> bool {
    let cannot_light = IS_SOLID | IS_LIT | CANT_LIGHT | IS_LIGHT;
    if grid.grid.contents[loc] & cannot_light != 0 {
        return false;
    }
    match grid.sight_lines.get(&loc) {
        Some(sl) => for lit_loc in sl.iter() {
            grid.grid.contents[*lit_loc] |= IS_LIT;
        },
        None => {return false;}
    }
    grid.grid.contents[loc] |= IS_LIGHT | IS_LIT;
    true
}

pub fn precompute_data(grid: Grid) -> GridData {
    let mut sight_lines = HashMap::new();
    for i in 0..(grid.height * grid.width) {
        if grid.contents[i as usize] & IS_SOLID == 0 {
            sight_lines.insert(i as usize, get_sight_line(&grid, i));
        }
    }
    GridData {grid: grid, sight_lines: sight_lines}
}

pub fn get_sight_line(grid: &Grid, idx: i32) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity((grid.height + grid.width - 1) as usize);
    
    let mut right_idx = idx + 1;
    while right_idx % grid.width != 0 {
        if grid.contents[right_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(right_idx as usize);
        right_idx += 1;
    }

    let mut left_idx = idx - 1;
    while left_idx % grid.width != grid.width - 1 && left_idx >= 0 {
        if grid.contents[left_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(left_idx as usize);
        left_idx -= 1;
    }

    let mut up_idx = idx - grid.width;
    while up_idx >= 0 {
        if grid.contents[up_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(up_idx as usize);
        up_idx -= grid.width;
    }

    let mut down_idx = idx + grid.width;
    while down_idx < grid.height * grid.width {
        if grid.contents[down_idx as usize] & IS_SOLID != 0 {
            break;
        }
        result.push(down_idx as usize);
        down_idx += grid.width;
    }

    result.sort();
    result
}

pub fn get_grid_from_string(input: &str, height: i32, width: i32) -> Result<Grid, String> {
    let mut data: Vec<u8> = Vec::with_capacity((height * width) as usize);
    let cleaned_input = input.replace(char::is_whitespace, "");
    if cleaned_input.len() != (height * width) as usize {
        return Err(format!("Input length mismatch: got {} significant squares and expected {}",
                           cleaned_input.len(), height * width));
    }

    for c in cleaned_input.as_bytes() {
        data.push(match *c {
            b'X' => IS_SOLID,
            b'^' => CANT_LIGHT,
            b'*' => IS_LIGHT,
            b'#' => IS_LIT,
            b'0' => IS_SOLID | IS_CONSTRAINED,
            b'1' => 1 | IS_SOLID | IS_CONSTRAINED,
            b'2' => 2 | IS_SOLID | IS_CONSTRAINED,
            b'3' => 3 | IS_SOLID | IS_CONSTRAINED,
            b'4' => 4 | IS_SOLID | IS_CONSTRAINED,
            _ => 0
        });
    }
    Ok(Grid {contents: data, height: height, width: width})
}

pub fn print_griddata_to_string(grid: &GridData, pretty_print: bool) -> String {
    print_grid_to_string(&grid.grid, pretty_print)
}

pub fn print_grid_to_string(grid: &Grid, pretty_print: bool) -> String {
    let dim = (grid.height * grid.width) as usize;
    let mut s = String::with_capacity(dim);
    for idx in 0..dim {
        let val = grid.contents[idx];
        if pretty_print && ((idx as i32) % grid.width == 0) {
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
