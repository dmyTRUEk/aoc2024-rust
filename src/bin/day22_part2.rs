//! Advent of Code - Day 22 solution.

#![deny(
	unused_assignments,
	unused_mut,
	unused_results,
	unused_variables,
)]

use std::{fs, iter::repeat_n};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};


fn main() {
	let solution = solve_file("./input/day22_input.txt");
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
	let ss = parse_input(&text);
	let sss: Vec<Vec<u64>> = ss.iter().map(|&s| evolve_seq(2000, s)).collect();
	let dss: Vec<Vec<i64>> = sss.iter().map(|ss| deltas(ss)).collect();

	// single-threaded:
	// let mut max_bananas: u64 = 0;
	// for ds1 in -9 ..= 9 {
	// 	for ds2 in -9 ..= 9 {
	// 		for ds3 in -9 ..= 9 {
	// 			for ds4 in -9 ..= 9 {
	// 				let ds_4 = [ds1, ds2, ds3, ds4];
	// 				println!("ds_4 = {ds_4:?}");
	// 				// println!("\n\n\nds_4 = {ds_4:?}");
	// 				let bananas = bananas_with_ds4_total(&sss, &dss, ds_4);
	// 				// if bananas >= 23 {
	// 				// 	println!("ds_4:\t{ds1}\t{ds2}\t{ds3}\t{ds4}");
	// 				// 	println!("bananas = {bananas}");
	// 				// }
	// 				max_bananas = max_bananas.max(bananas);
	// 				// if ds_4 == [-2,1,-1,3] { return max_bananas }
	// 				// if ds_4 == [-3,0,-1,-3] { return max_bananas }
	// 			}
	// 		}
	// 	}
	// }
	// max_bananas

	// mutli-threaded:
	// let max_bananas: Vec<u64> = (-9 ..= 9)
	// 	.into_par_iter()
	// 	.map(|ds1| {
	// 		// println!("ds1:\t{ds1}");
	// 		let mut max_bananas: u64 = 0;
	// 		for ds2 in -9 ..= 9 {
	// 			println!("ds12:\t{ds1}\t{ds2}");
	// 			for ds3 in -9 ..= 9 {
	// 				for ds4 in -9 ..= 9 {
	// 					let ds_4 = [ds1, ds2, ds3, ds4];
	// 					let bananas = bananas_with_ds4_total(&sss, &dss, ds_4);
	// 					max_bananas = max_bananas.max(bananas);
	// 				}
	// 			}
	// 		}
	// 		max_bananas
	// 	})
	// 	.collect();
	// max_bananas.into_iter().max().unwrap()

	// mutli-threaded with itertools:
	repeat_n(-9 ..= 9, 4)
		.multi_cartesian_product()
		.par_bridge()
		.map(|ds_4| {
			// println!("ds_4 = {ds_4:?}");
			bananas_with_ds4_total(&sss, &dss, ds_4.try_into().unwrap())
		})
		.max()
		.unwrap()
}

fn parse_input(input: &str) -> Vec<u64> {
	input
		.trim()
		.split('\n')
		.map(|l| l.parse().unwrap())
		.collect()
}

fn evolve_seq(n: u64, mut s: u64) -> Vec<u64> {
	let mut seq = vec![];
	for _ in 0..n {
		seq.push(s);
		s = evolve(s);
	}
	seq
}

fn evolve(mut s: u64) -> u64 {
	s = ((s << 6) ^ s) % 16777216; // | (1 <<< 24)
	s = ((s >> 5) ^ s) % 16777216;
	s = ((s << 11)^ s) % 16777216;
	s
}

fn deltas(ss: &Vec<u64>) -> Vec<i64> {
	ss.iter()
		.map(|s| s % 10)
		.collect::<Vec<u64>>()
		.windows(2)
		.map(|ab| {
			let [a, b] = ab else { unreachable!() };
			let a: i64 = *a as i64;
			let b: i64 = *b as i64;
			b - a
		})
		.collect()
}

fn bananas_with_ds4_total(sss: &Vec<Vec<u64>>, dss: &Vec<Vec<i64>>, ds_4: [i64; 4]) -> u64 {
	sss.iter().zip(dss)
		.map(|(ss, ds)| bananas_with_ds4(ss, ds, ds_4))
		.sum()
}

fn bananas_with_ds4(ss: &Vec<u64>, ds: &Vec<i64>, ds_4: [i64; 4]) -> u64 {
	let index = ds.windows(4).position(|abcd| abcd == ds_4);
	index.map(|i| ss[i+4] % 10).unwrap_or(0)
}



#[test]
fn bananas_with_ds4_() {
	let ss = evolve_seq(10, 123);
	let ds = deltas(&ss);
	let ds_4 = [-1, -1, 0, 2];
	assert_eq!(
		6,
		bananas_with_ds4(&ss, &ds, ds_4)
	)
}

#[test]
fn evolve_seq_() {
	assert_eq!(
		vec![123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432],
		evolve_seq(10, 123)
	)
}

#[test]
fn deltas_() {
	assert_eq!(
		vec![-3, 6, -1, -1, 0, 2, -2, 0, -2],
		deltas(&evolve_seq(10, 123))
	)
}

// #[test]
// fn example_1() {
// 	assert_eq!(
// 		37327623,
// 		solve_part_one("./input/day22_example_1.txt")
// 	)
// }

#[test]
fn example_2() {
	assert_eq!(
		23,
		solve_file("./input/day22_example_2.txt")
	)
}
