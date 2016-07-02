use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED};
use super::{GridData};
use super::utils::*;

// The numbering scheme for neighbors and corners looks like this:
// 4 0 5
// 3 X 1
// 7 2 6
// where X is the location in question.

const CORNER_RULE_LUT_1: [([bool; 4], [u8; 4]); 4] = [
    ([true, true, false, false], [5, 255, 255, 255]),
    ([false, true, true, false], [6, 255, 255, 255]),
    ([false, false, true, true], [7, 255, 255, 255]),
    ([true, false, false, true], [4, 255, 255, 255]),
];

const CORNER_RULE_LUT_2: [([bool; 4], [u8; 4]); 4] = [
    ([true, true, true, false], [5, 6, 255, 255]),
    ([false, true, true, true], [6, 7, 255, 255]),
    ([true, false, true, true], [4, 7, 255, 255]),
    ([true, true, false, true], [4, 5, 255, 255]),
];

fn apply_zero_rule(grid: &mut GridData, loc: usize) -> bool {
    let positions = &get_neighbors(grid, loc).1[0..4];
    for &position in positions.iter() {
        if position != super::INVALID_POSITION {
            grid.grid.contents[position] |= CANT_LIGHT;
        }
    }
    true
}

fn apply_number_light_rule(grid: &mut GridData, loc: usize) -> bool {
    let (_valid, _positions) = get_neighbors(grid, loc);
    let (valid, positions) = (&_valid[..4], &_positions[..4]);
    let num_mask: u8 = 0x7;
    let num_valid: u8 = valid.iter().fold(0, |a, &i| if i {a + 1} else {a});
    let effective_constraint_num = grid.grid.contents[loc] & num_mask
        - count_surrounding_lights(&grid.grid.contents, &positions);

    if grid.grid.contents[loc] & num_mask == num_valid {
        for (should_consider, &position) in valid.iter().zip(positions.iter()) {
            if *should_consider {
                insert_light(grid, position);
            }
        }
        return true;
    }
    false
}

fn apply_number_corner_rule(grid: &mut GridData, loc: usize) -> bool {
    let (valid, positions) = get_neighbors(grid, loc);
    let num_mask: u8 = 0x7;
    let num_valid: u8 = valid.iter().fold(0, |a, &i| if i {a + 1} else {a});
    if (grid.grid.contents[loc] & num_mask) + 1 == num_valid {
        return true;
    } else {
        return false;
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
