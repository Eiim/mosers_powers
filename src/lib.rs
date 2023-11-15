use rug::Integer;
use std::mem::replace;

pub fn calc_to_from(min: u32, max: u32, nstart: Integer) {
	let mut n: Integer = nstart;
	let mut x: u32 = min;
	let mut qrt2_iter: u32 = 20;
	let mut qrt2_frac = qrt2(qrt2_iter);
	let mut delta = 0;
	loop {
		n *= &qrt2_frac.0;
		n /= &qrt2_frac.1;
		x += 1;
		delta = 0;
		
		let target = Integer::from(1u8) << x;
		
		let mut prev_sign = 0i8;
		loop {
			let curr = calc_fact(&n);
			println!("Found sequence value {0}: {1}", &n, &curr);
			if curr == target {
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
		
		if delta > 2 {
			qrt2_iter += 20;
			//println!("Increasing qrt2 iterations to {0}", &qrt2_iter);
			qrt2_frac = qrt2(qrt2_iter);
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

/*
 * Uses a simple sequence of the generalized continued fraction
 * Very fast to calculate with high precision
 * Ideally we'd store the results for later but that's unnecessary for now
 * https://en.wikipedia.org/wiki/Generalized_continued_fraction#Roots_of_positive_numbers
*/
pub fn qrt2(iter: u32) -> (Integer, Integer) {
	let mut A2: Integer = Integer::from(1u8);
	let mut B2: Integer = Integer::from(0u8);
	let mut A1: Integer = Integer::from(1u8);
	let mut B1: Integer = Integer::from(1u8);
	for i in 1..iter {
		let a: u32 = 1+2*(&i-1);
		let mut b = 2;
		if i % 2 == 1 {
			b = 4*&i;
		}
		let A = b * A1.clone() + a * A2.clone();
		let B = b * B1.clone() + a * B2.clone();
		//println!("i: {4} a: {0} b: {1} A: {2} B: {3}", &a, &b, &A, &B, &i);
		A2 = replace(&mut A1, A);
		B2 = replace(&mut B1, B);
	}
	return(A1, B1)
}