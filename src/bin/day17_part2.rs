//! hell yeah

use std::collections::HashMap;



// const A_START: u64 = 35_184_370_000_000;
// const A_START: u64 = 10_000_101_000_091;
// const A_START: u64 = 35_188_251_188_216;
const A_START: u64 = 40 * T;

fn main() {
	// let _ = u64::MAX;
	let min_reg_a = find_min_reg_a(A_START);
	println!("min_reg_a = {min_reg_a}");
}

const PROGRAM: [u8; 16] = [2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0];

fn find_min_reg_a(a_start: u64) -> u64 {
	let mut a = a_start;
	let mut res;
	// let mut cache = Cache::new();
	loop {
		// res = exe_real_input_nc(a);
		// (res, cache) = exe_real_input_yc(a, cache);
		res = exe_real_input_nc_loop(a);
		if a % 1_000_001 == 0 {
			println!("a = {a}");
			println!("pr  = {PROGRAM:?}");
			println!("res = {res:?}");
		}
		if res == PROGRAM { return a; }
		a += 1;
		// if a > A_START + 1*G { return 0 }
	}
}



fn exe_real_input_nc(a_org: u64) -> Vec<u8> {
	let a = a_org;
	let b = a % 8;
	let b = b ^ 1;
	let c = a >> b;
	let b = b ^ c;
	let a = a >> 3;
	let b = b ^ 6;
	let b_mod_8: u8 = (b % 8) as u8;
	if a == 0 {
		vec![b_mod_8]
	} else {
		let mut res = exe_real_input_nc(a);
		res.insert(0, b_mod_8);
		res
	}
}


fn exe_real_input_nc_loop(a_org: u64) -> Vec<u8> {
	let mut a: u64 = a_org;
	let mut b: u64;
	let mut c: u64;
	let mut output = Vec::<u8>::new();
	loop {
		b = a % 8;
		b = b ^ 1;
		c = a >> b;
		b = b ^ c;
		a = a >> 3;
		b = b ^ 6;
		let b_mod_8: u8 = (b % 8) as u8;
		output.push(b_mod_8);
		if a == 0 { return output }
	}
}


type Cache = HashMap<u64, Vec<u8>>;

fn exe_real_input_yc(a_org: u64, cache: Cache) -> (Vec<u8>, Cache) {
	// dbg_trace "exe_real_input -> a = {a_org}"
	if let Some(res) = cache.get(&a_org) {
		// dbg_trace "CACHE HIT"
		(res.to_vec(), cache)
	} else {
		// dbg_trace "CACHE MISS"
		let a = a_org;
		let b = a % 8;
		let b = b ^ 1;
		let c = a >> b;
		let b = b ^ c;
		let a = a >> 3;
		let b = b ^ 6;
		let b_mod_8: u8 = (b % 8) as u8;
		let (mut res, mut cache) = if a == 0 {
			(vec![b_mod_8], cache)
		} else {
			let (mut res, cache) = exe_real_input_yc(a, cache);
			res.insert(0, b_mod_8);
			(res, cache)
		};
		if a_org < 10 * G {
			res.shrink_to_fit();
			cache.insert(a_org, res.clone());
		}
		// dbg_trace "exe_real_input -> res = {res}"
		(res, cache)
	}
}



const k: u64 = 1_000;
const M: u64 = k * k;
const G: u64 = M * k;
const T: u64 = G * k;
const P: u64 = T * k;
const E: u64 = P * k;
// const Z: u64 = E * k;



#[cfg(test)]
mod exe_real_input_nc {
	use super::*;
	#[test]
	fn _379009() {
		assert_eq!(
			exe_real_input_nc(379009),
			vec![7, 7, 1, 4, 1, 6, 7]
		);
	}
	#[test]
	fn _37900914542() {
		assert_eq!(
			exe_real_input_nc(37900914542),
			vec![7, 4, 1, 4, 7, 2, 5, 3, 0, 6, 4, 3]
		);
	}
}

#[cfg(test)]
mod exe_real_input_yc {
	use super::*;
	#[test]
	fn _379009() {
		assert_eq!(
			exe_real_input_yc(379009, HashMap::new()).0,
			vec![7, 7, 1, 4, 1, 6, 7]
		);
	}
	#[test]
	fn _37900914542() {
		assert_eq!(
			exe_real_input_yc(37900914542, HashMap::new()).0,
			vec![7, 4, 1, 4, 7, 2, 5, 3, 0, 6, 4, 3]
		);
	}
}

#[cfg(test)]
mod exe_real_input_nc_loop {
	use super::*;
	#[test]
	fn _379009() {
		assert_eq!(
			exe_real_input_nc_loop(379009),
			vec![7, 7, 1, 4, 1, 6, 7]
		);
	}
	#[test]
	fn _37900914542() {
		assert_eq!(
			exe_real_input_nc_loop(37900914542),
			vec![7, 4, 1, 4, 7, 2, 5, 3, 0, 6, 4, 3]
		);
	}
}

