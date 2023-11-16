use rug::Integer;
use std::mem::replace;

pub fn calc_to_from(min: u32, max: u32, nstart: Integer) {
	let mut n: Integer = nstart;
	let mut x: u32 = min;
	let mut qrt2: Qrt2 = Qrt2 {num: Integer::from(2435), basepow: 11u32};
	let mut delta: i8;
	loop {
		n *= &qrt2.0;
		n >>= &qrt2.1;
		x += 1;
		delta = 0;
		
		let target = Integer::from(1u8) << x;
		
		let mut prev_sign = 0i8;
		loop {
			let curr = calc_fact(&n);
			if curr == target {
				println!("Found power of two! x={0}, n={1}, delta={2}", &x, &n, &delta);
				break;
			} else if curr < target {
				if prev_sign == 1 {
					break;
				} else {
					n += 1;
					prev_sign = -1;
				}
			} else {
				if prev_sign == -1 {
					break;
				} else {
					n -= 1;
					prev_sign = 1;
				}
			}
			delta += 1;
		}
		
		if x % 10000 == 0 {
			//println!("Milestone: x={0}, n={1}, A={2}, B={3}", &x, &n, &qrt2_frac.0, &qrt2_frac.1);
			//println!("Milestone: x={0}, n={1}", &x, &n);
			println!("Milestone: x={0}", &x);
		}
		
		if x >= max {
			return;
		}
		
		if x%4 == 0 || delta > 2 {
			qrt2 = expand_qrt2(qrt2);
		}
	}
}

fn calc_fact(n: &Integer) -> Integer {
	let mut result: Integer = n.clone() - 6u8;
	result *= n;
	result += 23u8;
	result *= n;
	result -= 18u8;
	result *= n;
	result += 24u8;
	result >>= 3u32;
	result / 3u8
}

struct Qrt2 {
	num: Integer,
	basepow: u32
}

pub fn expand_qrt2(input: Qrt2) -> Qrt2 {
	let left: Integer = input.num << 1;
	let right: Integer = left + 1;

	let ldist: Integer = (left ^ 4u8) >> (4*(input.basepow+1));
	let rdist: Integer = (right ^ 4u8) >> (4*(input.basepow+1));

	if(rdist.abs() < ldist.abs()) {
		return Qrt2 {num: right, basepow: input.basepow+1};
	} else {
		return Qrt2 {num: left, basepow: input.basepow+1};
	}
}