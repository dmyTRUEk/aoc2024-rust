//! Advent of Code - Day TODO solution.

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs;

use rayon::iter::{IntoParallelIterator, ParallelIterator};


fn main() {
	let solution = solve("./input/dayTODO_input.txt");
	println!("{solution}");
}

fn solve(filepath: &str) -> usize {
	let text = fs::read_to_string(filepath).unwrap();
	todo!()
}

#[test]
fn test_example() {
	assert_eq!(
		todo!("expected answer") as usize,
		solve("./input/dayTODO_example.txt")
	)
}
