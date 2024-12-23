//! Advent of Code - Day TODO solution.

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs;

// use rayon::iter::{IntoParallelIterator, ParallelIterator};


fn main() {
	let solution = solve_file("./input/dayTODO_input.txt");
	println!("{solution}");
}

fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath);
	solve_text(&text)
}

fn read_file_to_string(filepath: &str) -> String {
	fs::read_to_string(filepath).unwrap()
}

fn solve_text(text: &str) -> u64 {
	let TODO = parse_input(&text);
	todo!()
}

fn parse_input(input: &str) -> (/*TODO*/) {
	todo!()
}



#[test]
fn example_1() {
	assert_eq!(
		todo!("expected answer") as u64,
		solve_file("./input/dayTODO_example_1.txt")
	)
}
