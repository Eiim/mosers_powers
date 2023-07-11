use num_bigint::BigUint;
use std::mem::replace;

pub fn calc_to_from(min: u32, max: u32, nstart: BigUint) {
	let mut n: BigUint = nstart;
	let mut x: u32 = min;
	let base: BigUint = BigUint::from(2u32);
	let mut qrt2_iter: u32 = 20;
	let mut qrt2_frac = qrt2(qrt2_iter);
	let one = BigUint::from(1u8);
	let mut delta = 0;
	loop {
		n *= &qrt2_frac.0;
		n /= &qrt2_frac.1;
		x += 1;
		delta = 0;
		
		let target = base.pow(x);
		
		let mut prev_sign = 0i8;
		loop {
			let curr = calc_fact(&n);
			//println!("curr: {0}, target: {1}", &curr, &target);
			if curr == target {
				println!("Found power of two! x={0}, n={1}, delta={2}", &x, &n, &delta);
				break;
			} else if curr < target {
				if prev_sign == 1 {
					//println!("Missed power of two. x={0}, n={1}, f={2}", &x, &n, &curr);
					println!("Missed power of two at x={0}", &x);
					break;
				} else {
					//println!("incrementing n");
					n += &one;
					prev_sign = -1;
				}
			} else {
				if prev_sign == -1 {
					//println!("Missed power of two. x={0}, n={1}, f={2}", &x, &n, &curr);
					println!("Missed power of two at x={0}", &x);
					break;
				} else {
					//println!("decrementing n");
					n -= &one;
					prev_sign = 1;
				}
			}
			delta += 1;
		}
		if x >= max {
			return;
		}
		
		if delta > 2 {
			qrt2_iter += 10;
			println!("Increasing qrt2 iterations to {0}", &qrt2_iter);
			qrt2_frac = qrt2(qrt2_iter);
		}
	}
}

fn calc_fact(n: &BigUint) -> BigUint {
	let one: BigUint = BigUint::from(1u32);
	let two: BigUint = BigUint::from(2u32);
	let three: BigUint = BigUint::from(3u32);
	let twenty_four: BigUint = BigUint::from(24u32);
	
	let mut sum_one: BigUint = n.clone();
	sum_one *= n-&one;
	sum_one *= n-&two;
	sum_one *= n-&three;
	sum_one /= &twenty_four;
	
	let mut sum_two: BigUint = n.clone();
	sum_two *= n-&one;
	sum_two = sum_two >> 1;
		
	sum_one + sum_two + &one
}

/*
 * Uses a simple sequence of the generalized continued fraction
 * Very fast to calculate with high precision
 * Ideally we'd store the results for later but that's unnecessary for now
 * https://en.wikipedia.org/wiki/Generalized_continued_fraction#Roots_of_positive_numbers
*/
pub fn qrt2(iter: u32) -> (BigUint, BigUint) {
	let mut A2: BigUint = BigUint::from(1u8);
	let mut B2: BigUint = BigUint::from(0u8);
	let mut A1: BigUint = BigUint::from(1u8);
	let mut B1: BigUint = BigUint::from(1u8);
	for i in 1..iter {
		let a: u32 = 1+2*(&i-1);
		let mut b = 2;
		if i % 2 == 1 {
			b = 4*&i;
		}
		let A = b * &A1 + a * &A2;
		let B = b * &B1 + a * &B2;
		//println!("i: {4} a: {0} b: {1} A: {2} B: {3}", &a, &b, &A, &B, &i);
		A2 = replace(&mut A1, A);
		B2 = replace(&mut B1, B);
	}
	return(A1, B1)
}