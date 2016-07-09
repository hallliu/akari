use super::*;
use super::{make_constraint_cnf_cache, make_constraint_cnf_generator};
use super::super::utils::*;

use std::collections::HashSet;
use std::collections::HashMap;

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
