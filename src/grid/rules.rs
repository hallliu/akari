use super::{IS_SOLID, IS_LIT, IS_LIGHT, CANT_LIGHT, IS_CONSTRAINED};
use super::{GridData};
use super::utils::*;

fn apply_zero_rule(grid: &mut GridData, loc: usize) {
    let (valid, positions) = get_adj_empties(grid, loc);
    for (should_consider, position) in valid.iter().zip(positions.iter()) {
        if *should_consider {
            grid.grid.contents[*position] |= CANT_LIGHT;
        }
    }
}
