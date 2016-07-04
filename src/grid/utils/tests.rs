use super::*;
use std::collections::HashSet;
use std::collections::HashMap;

static TEST_GRID_STR: &'static str = "
____X
X1__X
XX__X
X__21
_X___";

#[test]
fn test_get_adj_empties() {
    let test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    let mut cases = HashMap::new();
    let ip = super::super::INVALID_POSITION;
    cases.insert(0, ([false, true, false, false, false, false, false, false],
                     [ip, 1, 5, ip, ip, ip, 6, ip]));
    cases.insert(7, ([true, true, true, false, true, true, true, false],
                     [2, 8, 12, 6, 1, 3, 13, 11]));
    cases.insert(20, ([false, false, false, false, false, true, false, false],
                      [15, 1, ip, ip, ip, 16, ip, ip]));
    for (pos, expected) in &cases {
        let (valid, positions) = get_neighbors(&test_grid, *pos);
        for idx in 0..valid.len() {
            assert_eq!(valid[idx], expected.0[idx]);
            if valid[idx] {
                assert_eq!(positions[idx], expected.1[idx]);
            }
        }
    }
}

#[test]
fn test_light_square() {
    let expected_result_grid = "
        ___#X
        X1#*X
        XX_#X
        X__21
        _X___".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    insert_light(&mut test_grid, 8);
    assert_eq!(&expected_result_grid, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_print_write_grid() {
    let test_grid = get_grid_from_string(TEST_GRID_STR, 5).unwrap();
    let result_str = print_grid_to_string(&test_grid, false);
    let expected = TEST_GRID_STR.replace(char::is_whitespace, "");
    assert_eq!(&expected, &result_str);
}

#[test]
fn test_sight_line() {
    let test_grid_1 = get_grid_from_string(TEST_GRID_STR, 5).unwrap();

    let test_locations = vec!(0, 1, 8, 20);
    let expected_sight_lines: Vec<HashSet<usize>> = vec!(
        vec!(1, 2, 3).into_iter().collect(),
        vec!(0, 2, 3).into_iter().collect(),
        vec!(3, 7, 13).into_iter().collect(),
        vec!().into_iter().collect());

    for (loc, expected) in test_locations.iter().zip(expected_sight_lines) {
        let sight_line: HashSet<usize> = get_sight_line(&test_grid_1, *loc).into_iter().collect();
        assert_eq!(&sight_line, &expected);
    }
}
