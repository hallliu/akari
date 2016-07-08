use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED};
use super::{GridData};
use super::utils::*;

#[cfg(test)]
mod tests;

// The numbering scheme for neighbors and corners looks like this:
// 4 0 5
// 3 X 1
// 7 2 6
// where X is the location in question.

const INVALID_RELATIVE_POSITION: u8 = 255;

static CORNER_RULE_LUT_1: [([bool; 4], [u8; 4]); 4] = [
    ([true, true, false, false], [5, 255, 255, 255]),
    ([false, true, true, false], [6, 255, 255, 255]),
    ([false, false, true, true], [7, 255, 255, 255]),
    ([true, false, false, true], [4, 255, 255, 255]),
];

static CORNER_RULE_LUT_2: [([bool; 4], [u8; 4]); 4] = [
    ([true, true, true, false], [5, 6, 255, 255]),
    ([false, true, true, true], [6, 7, 255, 255]),
    ([true, false, true, true], [4, 7, 255, 255]),
    ([true, true, false, true], [4, 5, 255, 255]),
];

static CORNER_RULE_LUT_3: [([bool; 4], [u8; 4]); 1] = [
    ([true, true, true, true], [4, 5, 6, 7])
];

pub fn apply_constraint_rule(grid: &mut GridData, loc: usize) -> bool {
    let (valid, positions) = get_neighbors(grid, loc);
    let (valid_4, positions_4) = (&valid[..4], &positions[..4]);
    let num_mask: u8 = 0x7;
    let num_valid: u8 = valid_4.iter().fold(0, |a, &i| if i {a + 1} else {a});
    let effective_constraint_num = grid.grid.contents[loc] & num_mask
        - count_surrounding_lights(&grid.grid.contents, &positions);

    if effective_constraint_num == 0 {
        mark_rel_positions(grid, &[0, 1, 2, 3], &positions, CANT_LIGHT);
        return true;
    } else if effective_constraint_num == num_valid {
        apply_number_light_rule(grid, valid_4, positions_4);
        return true;
    } else if effective_constraint_num + 1 == num_valid {
        apply_number_corner_rule(grid, effective_constraint_num, &valid, &positions);
        return true;
    }
    false 
}

fn apply_number_light_rule(grid: &mut GridData, valid: &[bool], positions: &[usize]) {
    for (&should_consider, &position) in valid.iter().zip(positions.iter()) {
        if should_consider {
            insert_light(grid, position);
        }
    }
}

fn mark_rel_positions(grid: &mut GridData, rel_positions: &[u8],
                      abs_positions: &[usize], mark: u8) {
    for &relpos in rel_positions.iter() {
        if relpos == INVALID_RELATIVE_POSITION {
            continue;
        }
        let ap = abs_positions[relpos as usize];
        if ap == super::INVALID_POSITION {
            continue;
        }

        grid.grid.contents[ap] |= mark;
    }
}

fn apply_number_corner_rule(grid: &mut GridData, effective_constraint_num: u8,
                            valid: &[bool; 8], positions: &[usize; 8]) {
    let mut apply_corner_lut = |lut: &[([bool; 4], [u8; 4])]| -> () {
        for &entry in lut.iter() {
            if entry.0 == &valid[..4] {
                mark_rel_positions(grid, &entry.1, positions, CANT_LIGHT);
                break;
            }
        }
    };

    match effective_constraint_num {
        1 => apply_corner_lut(&CORNER_RULE_LUT_1),
        2 => apply_corner_lut(&CORNER_RULE_LUT_2),
        3 => apply_corner_lut(&CORNER_RULE_LUT_3),
        _ => { }
    }
}

fn count_surrounding_lights(grid_contents: &Vec<u8>, adjacents: &[usize]) -> u8 {
    adjacents.iter().fold(0, |a, &i| {
        if i != super::INVALID_POSITION && grid_contents[i] & IS_LIGHT != 0 {
            a + 1
        } else {
            a
        }
    })
}

pub fn apply_spatial_rule(grid: &mut GridData, loc: usize) -> bool {
    if grid.grid.contents[loc] & (IS_LIT | IS_LIGHT) != 0 {
        return false;
    }
    let sl = match get_filtered_sight_line(grid, loc) {
        Some(x) => x,
        None => { return false; }
    };
    if sl.len() == 0 && grid.grid.contents[loc] & CANT_LIGHT == 0 {
        insert_light(grid, loc);
        return true;
    } else if sl.len() == 1 && grid.grid.contents[loc] & CANT_LIGHT != 0 {
        insert_light(grid, sl[0]);
        return true;
    } else if sl.len() == 2 && grid.grid.contents[loc] & CANT_LIGHT != 0 {
        match compute_sight_corner_rule(grid, &sl, loc) {
            Some(x) => {
                grid.grid.contents[x] |= CANT_LIGHT;
                return true;
            },
            None => { return false; }
        }
    }

    return false;
}

fn get_filtered_sight_line(grid: &GridData, loc: usize) -> Option<Vec<usize>> {
    match grid.sight_lines.get(&loc) {
        Some(sl) => 
            Some(sl.into_iter()
                .filter(|&&x| grid.grid.contents[x] & (CANT_LIGHT | IS_LIT) == 0)
                .map(usize::clone)
                .collect()),
        None => None
    }
}

fn compute_sight_corner_rule(grid: &GridData, valid_sl: &Vec<usize>, loc: usize) -> Option<usize> {
    if valid_sl.len() != 2 {
        return None;
    }
    let diff0 = valid_sl[0] as i32 - loc as i32;
    let diff1 = valid_sl[1] as i32 - loc as i32;
    if (diff0.abs() < grid.grid.size && diff1.abs() < grid.grid.size) ||
        (diff0.abs() % grid.grid.size == 0 && diff1.abs() % grid.grid.size == 0) {
        return None;
    }
    let potential_mark = (loc as i32 + diff0 + diff1) as usize;
    let sight_of_potential_mark = match grid.sight_lines.get(&potential_mark) {
        Some(v) => v,
        None => { return None; }
    };
    match (sight_of_potential_mark.binary_search(&valid_sl[0]),
        sight_of_potential_mark.binary_search(&valid_sl[1])) {
        (Ok(_), Ok(_)) => { return Some(potential_mark); }
        _ => { return None; }
    }
}
