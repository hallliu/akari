use std::collections::HashMap;

pub mod utils;
pub mod rules;

const IS_SOLID: u8 = 1<<3;
const IS_LIT: u8 = 1<<4;
const IS_LIGHT: u8 = 1<<5;
const CANT_LIGHT: u8 = 1<<6;
const IS_CONSTRAINED: u8 = 1<<7;

pub struct Grid {
    contents: Vec<u8>,
    size: i32 
}

pub struct GridData {
    grid: Grid,
    sight_lines: HashMap<usize, Vec<usize>>
}
