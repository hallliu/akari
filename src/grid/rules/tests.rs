use super::*;
use super::super::utils::{
    get_grid_from_string,
    print_grid_to_string,
    precompute_data
};

static TEST_GRID_STR: &'static str = 
"____0
 X1__X
 XX__X
 0__21
 _X___";

#[test]
fn test_zero_rule() {
    let expected_result =
    "___^0
     X1__X
     XX__X
     0^_21
     ^X___".replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());

    apply_constraint_rule(&mut test_grid, 4);
    apply_constraint_rule(&mut test_grid, 15);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_number_light_rule() {
    let expected_result =
    "____0
     X1__X
     XX__X
     0__21
     _X##*".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    apply_constraint_rule(&mut test_grid, 19);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_number_light_rule_multi() {
    let expected_result =
    "#*##0
     X1##X
     XX#*X
     0#*21
     _X##*".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    apply_constraint_rule(&mut test_grid, 19);
    apply_constraint_rule(&mut test_grid, 18);
    apply_constraint_rule(&mut test_grid, 6);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_number_corner_rule() {
    let expected_result =
    "__^_0
     X1__X
     XX^_X
     0__21
     _X^__".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    apply_constraint_rule(&mut test_grid, 18);
    apply_constraint_rule(&mut test_grid, 6);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}
