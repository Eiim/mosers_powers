use rug::Integer;
use rug::ops::Pow;
use std::time::Instant;
use std::fs;

pub fn calc_to_from(min: u32, max: u32, nstart: Integer, qrt_start: Qrt2, file: bool) -> (u32, Integer, Qrt2, u128) {
	let mut n: Integer = nstart;
	let mut x: u32 = min;
	let mut qrt2 = qrt_start;
	let mut delta: i8;
	let mut start = Instant::now();
	loop {
		n *= &qrt2.num;
		n >>= &qrt2.basepow;
		n += 1; // Round up
		
		x += 1;
		delta = 0;
		
		let target = Integer::from(1u8) << x;
		
		let mut prev_sign = 0i8;
		loop {
			let curr = calc_fact(&n);
			if curr == target {
				println!("Found power of two! x={0}, n={1}, delta={2}", &x, &n, &delta);
				if file {
					message_power(&x);
				}
				return (x, n, qrt2, start.elapsed().as_millis());
			} else if curr < target {
				if prev_sign == 1 {
					break;
				} else {
					n += 1;
					prev_sign = -1;
				}
			} else {
				if prev_sign == -1 {
					n -= 1;
					break;
				} else {
					n -= 1;
					prev_sign = 1;
				}
			}
			delta += 1;
		}
		
		if x == max {
			return (x, n, qrt2, start.elapsed().as_millis());
		}
		
		if x%4 == 0 {
			qrt2 = expand_qrt2(qrt2);
		}

		if delta > 2 { // Hopefully delta never exceeds 2, but if it does we might try an extra iteration
			println!("delta={0}, expanding 2^1/4", &delta);
			qrt2 = expand_qrt2(qrt2);
		}

		if x == max-1 {
			start = Instant::now();
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

pub struct Qrt2 {
	pub num: Integer,
	pub basepow: u32
}

fn expand_qrt2(input: Qrt2) -> Qrt2 {
	let center: Integer = input.num.clone() << 1;
	let target: Integer = Integer::from(1u8) << (4*(input.basepow+1))+1;
	let csq: Integer = center.clone()*center.clone();
	let ccbd = csq.clone()*center.clone();
	let cquad: Integer = center.clone().pow(4);
	
	if cquad > target {
		//println!("High");
		let lquad: Integer = cquad.clone() - (ccbd << 2u32) + (csq*6u8) - (center.clone() << 2u32) + 1;
		let d: Integer = target.clone() - lquad;
		if d < 0 {
			println!("Pow 2 assumption broke high (basepow={0})", input.basepow+1);
			return expand_qrt2(Qrt2 {num: input.num-1, basepow: input.basepow});
		} else {
			if d < cquad.clone()-target {
				return Qrt2 {num: center-1, basepow: input.basepow+1}
			} else {
				return Qrt2 {num: center, basepow: input.basepow+1}
			}
		}
	} else {
		//println!("Low");
		let rquad: Integer = cquad.clone() + (ccbd << 2u32) + (csq*6u8) + (center.clone() << 2u32) + 1;
		let d: Integer = rquad - target.clone();
		if d < 0 {
			//println!("Pow 2 assumption broke low (basepow={0})", input.basepow+1);
			return expand_qrt2(Qrt2 {num: input.num+1, basepow: input.basepow});
		} else {
			if d < cquad.clone()-target {
				return Qrt2 {num: center+1, basepow: input.basepow+1}
			} else {
				return Qrt2 {num: center, basepow: input.basepow+1}
			}
		}
	}
}

pub fn message_cp(x: &u32) {
	let folder = "./messages/";
	let prefix = "";
	let data = format!("{} New mosers_powers milestone: x={}", prefix, x);
	fs::write(format!("{}cp-{}.txt", folder, x), data).expect(&format!("Can't write message to {}cp-{}.txt", folder, x));
}

pub fn message_power(x: &u32) {
	let folder = "./messages/";
	let prefix = "";
	let data = format!("{} Found power of 2 in A000127: x={}. See checkpoint file for details.", prefix, x);
	fs::write(format!("{}pow-{}.txt", folder, x), data).expect(&format!("Can't write message to {}pow-{}.txt", folder, x));
}