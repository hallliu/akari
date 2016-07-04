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
 1__21
 _X___";

#[test]
fn test_zero_rule() {
    let expected_result =
    "___^0
     X1__X
     XX__X
     1__21
     _X___".replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());

    apply_constraint_rule(&mut test_grid, 4);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_number_light_rule() {
    let expected_result =
    "____0
     X1__X
     XX__X
     1__21
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
     1#*21
     *X##*".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    apply_constraint_rule(&mut test_grid, 19);
    apply_constraint_rule(&mut test_grid, 18);
    apply_constraint_rule(&mut test_grid, 6);
    apply_constraint_rule(&mut test_grid, 15);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_number_corner_rule_1() {
    let expected_result =
    "__^_0
     X1__X
     XX^_X
     1__21
     _X^__".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    apply_constraint_rule(&mut test_grid, 18);
    apply_constraint_rule(&mut test_grid, 6);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_number_corner_rule_2() {
    let test_grid_str =
    "_____
     _____
     __3__
     ___X_
     _____";
    let expected_result = 
    "_____
     _^_^_
     __3__
     _^_X_
     _____".replace(char::is_whitespace, "");

    let mut test_grid = precompute_data(get_grid_from_string(test_grid_str, 5).unwrap());
    apply_constraint_rule(&mut test_grid, 12);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_isolated_square_1() {
    let expected_result =
    "____0
     X1__X
     XX__X
     1__21
     *X___".replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(TEST_GRID_STR, 5).unwrap());
    apply_spatial_rule(&mut test_grid, 20);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_isolated_square_2() {
    let test_grid_str =
    "^_^^0
     X1__X
     XX__X
     1__21
     _X___";

    let expected_result =
    "#*##0
     X1__X
     XX__X
     1__21
     _X___".replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(test_grid_str, 5).unwrap());
    apply_spatial_rule(&mut test_grid, 0);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_spatial_corner_rule_1() {
    let test_grid_str =
    "___^0
     X1__X
     XX_^X
     1__21
     _X___";

    let expected_result =
    "___^0
     X1^_X
     XX_^X
     1__21
     _X___".replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(test_grid_str, 5).unwrap());
    apply_spatial_rule(&mut test_grid, 13);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_spatial_corner_rule_2() {
    let test_grid_str =
    "____0
     X1_^X
     XX_^X
     1__21
     _X___";

    let expected_result =
    "__^_0
     X1_^X
     XX_^X
     1__21
     _X___".replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(test_grid_str, 5).unwrap());
    apply_spatial_rule(&mut test_grid, 13);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}

#[test]
fn test_spatial_corner_rule_negative() {
    let test_grid_str =
    "____0
     X1_^X
     X_^^X
     1__21
     _X___";

    let expected_result = test_grid_str.replace(char::is_whitespace, "");
    let mut test_grid = precompute_data(get_grid_from_string(test_grid_str, 5).unwrap());
    apply_spatial_rule(&mut test_grid, 13);

    assert_eq!(&expected_result, &print_grid_to_string(&test_grid.grid, false));
}
