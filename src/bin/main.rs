use std::env;
use advent_of_rust_2023::*;

pub fn main() {
    env::set_var("should_print", "true");
    solution_1::part_one();
    solution_1::part_two();
}
