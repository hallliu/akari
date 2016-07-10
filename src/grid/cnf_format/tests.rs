use super::*;
use super::{make_constraint_cnf_cache, make_constraint_cnf_generator};
use super::super::utils::*;

use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

static TEST_GRID_STR: &'static str = "
____X
X1__X
XX__X
X__21
_X___";

#[test]
fn test_make_cnf_cache() {
    let v: Vec<(u32, Vec<bool>)>  = make_constraint_cnf_cache(5);
    let mut seen: HashSet<Vec<bool>> = HashSet::new();
    for &(count, ref bv) in v.iter() {
        assert_eq!(5, bv.len());
        assert_eq!(count, bv.iter().fold(0, |a, &x| if x { a + 1 } else { a }));
        seen.insert(bv.clone());
    }
    assert_eq!(v.len(), seen.len());
}

#[test]
fn test_get_constraints_0() {
    let sat_ids = vec![];
    let disallowed_sums = vec![0];
    get_constraints_test_helper(sat_ids, 0, disallowed_sums);
}

#[test]
fn test_get_constraints_1() {
    let sat_ids = vec![1, 2, 4];
    let disallowed_sums = vec![1, -5, -3];
    get_constraints_test_helper(sat_ids, 2, disallowed_sums);
}

#[test]
fn test_get_constraints_2() {
    let sat_ids = vec![1, 2, 4, 8];
    let disallowed_sums = vec![13, 11, 7, -1];
    get_constraints_test_helper(sat_ids, 1, disallowed_sums);
}

#[test]
fn test_get_constraints_3() {
    let sat_ids = vec![1, 2, 4, 8];
    let disallowed_sums = vec![9, 5, -3, 3, -5, -9];
    get_constraints_test_helper(sat_ids, 2, disallowed_sums);
}

#[test]
fn test_get_constraints_4() {
    let sat_ids = vec![1, 2];
    let disallowed_sums = vec![1, -1];
    get_constraints_test_helper(sat_ids, 1, disallowed_sums);
}

#[test]
fn test_get_constraints_5() {
    let sat_ids = vec![1, 2, 4];
    let disallowed_sums = vec![-7];
    get_constraints_test_helper(sat_ids, 3, disallowed_sums);
}

fn get_constraints_test_helper(sat_ids: Vec<i32>, num_true: u32, disallowed_sums: Vec<i32>) {
    let gen = make_constraint_cnf_generator(4);
    let slen = sat_ids.len();
    let expval = 1 << sat_ids.len();

    let constraint_clauses: Vec<_> = gen.get_constraints(sat_ids, num_true).collect();
    assert!(constraint_clauses.iter().fold(true, |b, v| b && v.len() == slen));
    let constraint_clause_sums: Vec<_> = constraint_clauses.iter()
        .map(|x| x.iter().fold(0, |a, y| a + y)).collect();
    println!("{:?}", constraint_clauses);
    for s in disallowed_sums.iter() {
        assert!(!constraint_clause_sums.contains(s));
    }

    assert_eq!(expval, disallowed_sums.len() + constraint_clause_sums.len());
}

fn to_vec_of_sets<T>(vecs: Vec<Vec<T>>) -> Vec<HashSet<T>> 
    where T: Eq + Hash {
    vecs.into_iter().map(|v| v.into_iter().collect::<HashSet<_>>())
        .collect()
}

fn assert_vec_eq_up_to_order<T>(v1: Vec<T>, v2: Vec<T>) 
    where T: PartialEq {
    for item in v1.iter() {
        assert!(v2.contains(item));
    }
    for item in v2.iter() {
        assert!(v1.contains(item));
    }
}

#[test]
fn test_get_numerical_constraint_1() {
    let test_grid =
        "__X
         _2*
         __#";
    let grid = precompute_data(get_grid_from_string(test_grid, 3).unwrap());
    let gen = make_constraint_cnf_generator(4);
    let to_cnf = super::produce_variable_mapping(&grid).0;
    let collected_clauses =
        to_vec_of_sets(super::get_numerical_constraint_clauses(&grid, &gen, &to_cnf, 4)
                       .collect());
    let expected_clauses = to_vec_of_sets(
        vec![vec![2, 3, 5],
             vec![2, -3, -5],
             vec![-2, 3, -5],
             vec![-2, -3, 5],
             vec![-2, -3, -5]]);
    assert_vec_eq_up_to_order(expected_clauses, collected_clauses);
}

#[test]
fn test_get_numerical_constraint_2() {
    let test_grid =
        "_2_
         ___
         XXX";

    let grid = precompute_data(get_grid_from_string(test_grid, 3).unwrap());
    let gen = make_constraint_cnf_generator(4);
    let to_cnf = super::produce_variable_mapping(&grid).0;
    let collected_clauses =
        to_vec_of_sets(super::get_numerical_constraint_clauses(&grid, &gen, &to_cnf, 1)
                       .collect());
    let expected_clauses = to_vec_of_sets(
        vec![vec![1, 2, 4],
             vec![-1, 2, 4],
             vec![1, -2, 4],
             vec![1, 2, -4],
             vec![-1, -2, -4]]);

    assert_vec_eq_up_to_order(expected_clauses, collected_clauses);
}

#[test]
fn test_sight_line_clause() {
    let test_grid =
        "__X_
         _X*X
         __#X
         X_#^";

    let grid = precompute_data(get_grid_from_string(test_grid, 4).unwrap());
    let to_cnf = super::produce_variable_mapping(&grid).0;

    let sight_line_clause_1: HashSet<i32> =
        super::get_sight_line_clauses(&grid, &to_cnf, 8).into_iter().collect();
    let sight_line_clause_2: HashSet<i32> =
        super::get_sight_line_clauses(&grid, &to_cnf, 15).into_iter().collect();

    let expected_clause_1 = [1, 4, 5, 6].into_iter().cloned().collect::<HashSet<_>>();
    let expected_clause_2 = [7].into_iter().cloned().collect::<HashSet<_>>();

    assert_eq!(expected_clause_1, sight_line_clause_1);
    assert_eq!(expected_clause_2, sight_line_clause_2);
}

#[test]
fn test_get_cnf_ids_within_sight() {
    let test_grid =
        "__X_
         _X*X
         __#X
         X_#^";
    let grid = precompute_data(get_grid_from_string(test_grid, 4).unwrap());
    let (to_cnf, to_grid) = super::produce_variable_mapping(&grid);
    let result = super::get_cnf_ids_within_sight(&grid, &to_grid, &to_cnf);
    let expected_result = vec![(1, 2), (1, 4), (1, 5), (4, 5), (5, 6), (6, 7)]
        .into_iter().collect::<HashSet<_>>();
    assert_eq!(expected_result, result);
}
