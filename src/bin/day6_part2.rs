//! Advent of Code - Day 6 Part 2 solution.

#![deny(
	unused_assignments,
	unused_results,
)]

use std::{collections::HashSet, fs};

use rayon::iter::{IntoParallelIterator, ParallelIterator};


fn main() {
	let solution = solve("./input/day6_input.txt");
	println!("{solution}");
}

fn solve(filepath: &str) -> usize {
	let text = fs::read_to_string(filepath).unwrap();
	let m = text_to_vec2d(text);
	let guard_pos = get_guard_state(&m).unwrap().to_pos();
	let trace_positions = get_trace_positions(m.clone(), guard_pos);
	trace_positions
		// .into_iter()
		.into_par_iter()
		.filter(|p| *p != guard_pos)
		.filter(|p| {
			let mut m = m.clone();
			m[p.y as usize][p.x as usize] = '#';
			is_looped(m, guard_pos)
		})
		.count()
}

type Map = Vec<Vec<char>>;

fn is_looped(mut m: Map, mut guard_pos: Pos) -> bool {
	let mut states: HashSet<GuardState> = HashSet::new();
	loop {
		let Some(dir) = get_guard_dir(&m, guard_pos) else { return false };
		let guard_state = GuardState { x: guard_pos.x, y: guard_pos.y, dir };
		if states.contains(&guard_state) { return true }
		let _ = states.insert(guard_state);
		guard_pos = step(&mut m, guard_pos);
	}
}

fn get_trace_positions(mut m: Map, mut guard_pos: Pos) -> Vec<Pos> {
	// assert!(!is_looped(m.clone()));
	let mut positions = HashSet::<Pos>::new();
	loop {
		let Some(guard_state) = get_guard_state_optim(&m, guard_pos) else {
			return positions.into_iter().collect()
		};
		let p = guard_state.to_pos();
		if !positions.contains(&p) {
			let _ = positions.insert(p);
		}
		guard_pos = step(&mut m, p);
	}
}

/// returns new guard position
fn step(m: &mut Map, pos@Pos{x, y}: Pos) -> Pos {
	let dir = get_guard_dir(m, pos).unwrap();
	let next_x = x + match dir {
		'<' => -1,
		'>' => 1,
		'^' | 'v' => 0,
		_ => unreachable!()
	};
	let next_y = y + match dir {
		'^' => -1,
		'v' => 1,
		'<' | '>' => 0,
		_ => unreachable!()
	};
	if is_blocked_at(m, Pos{x: next_x, y: next_y}) {
		m[y as usize][x as usize] = next_dir(dir);
		pos
	} else {
		m[y as usize][x as usize] = '.';
		// m[next_y][next_x] = dir;
		if !(next_x < 0 || next_y < 0) {
			if let Some(new_pos) = m.get_mut(next_y as usize).map(|l| l.get_mut(next_x as usize)).flatten() {
				*new_pos = dir;
			}
		}
		Pos { x: next_x, y: next_y }
	}
}

const DIRS: [char; 4] = ['^', '>', 'v', '<'];

fn next_dir(dir: char) -> char {
	DIRS[(DIRS.iter().position(|&d| d == dir).unwrap() + 1) % 4]
}

fn is_dir(c: &char) -> bool {
	DIRS.contains(c)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: i32, y: i32 }

fn is_blocked_at(m: &Map, Pos{x, y}: Pos) -> bool {
	if x < 0 || y < 0 { return false }
	m.get(y as usize)
		.map(|l| l.get(x as usize))
		.flatten()
		.is_some_and(|&c| c == '#')
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GuardState { x: i32, y: i32, dir: char }
impl GuardState {
	fn to_pos(&self) -> Pos {
		Pos { x: self.x, y: self.y }
	}
}

fn get_guard_state_optim(m: &Map, Pos{x, y}: Pos) -> Option<GuardState> {
	if x < 0 || y < 0 { return None }
	let Some(&dir) = m.get(y as usize).map(|l| l.get(x as usize)).flatten() else { return None };
	assert!(is_dir(&dir));
	Some(GuardState { x, y, dir })
}

fn get_guard_dir(m: &Map, Pos{x, y}: Pos) -> Option<char> {
	if x < 0 || y < 0 { return None }
	let Some(&dir) = m.get(y as usize).map(|l| l.get(x as usize)).flatten() else { return None };
	Some(dir)
}

/// returns X, Y, Direction
#[deprecated = "if possible, use optimized version: `get_guard_dir` or `get_guard_state_optim`"]
fn get_guard_state(m: &Map) -> Option<GuardState> {
	m.iter().enumerate()
		.filter_map(|(y, l)| l.iter().enumerate()
			.find(|(_x, c)| is_dir(c))
			.map(|(x, &c)| GuardState { x: x as i32, y: y as i32, dir: c })
		)
		.next()
}

fn text_to_vec2d(text: String) -> Map {
	text
		.lines()
		.map(|l| l.chars().collect())
		.collect()
}

fn map_to_string(m: &Map) -> String {
	m.iter().map(|l| l.iter().collect::<String>())
		.collect::<Vec<String>>()
		.join("\n")
}


#[test]
fn test_example() {
	assert_eq!(
		6,
		solve("./input/day6_example.txt")
	)
}
