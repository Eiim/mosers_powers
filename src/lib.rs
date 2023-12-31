pub mod io;

use rug::Integer;
use rug::ops::Pow;
use lazy_static::lazy_static;

pub struct Qrt2 {
	pub num: Integer,
	pub basepow: u32
}

pub fn calc_fact(n: &Integer) -> Integer {
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

// 8a0 = -28
// 8a1 = 255
// 8a2 = -396
// 64a3 = 37491
// y = (8n+8a0)n+8a1
// f(n) = ((y+8n+8a2)y+64a3)/(24*64)

lazy_static! {
	pub static ref ALPHA0: Integer = Integer::from(-28);
	pub static ref ALPHA1: Integer = Integer::from(255);
	pub static ref ALPHA2: Integer = Integer::from(-396);
	pub static ref ALPHA3: Integer = Integer::from(37491);
}

pub fn calc_fact_2(n: &Integer) -> Integer {
	let mut y: Integer = Integer::from(n << 3);
	let mut res: Integer = y.clone();
	y += &*ALPHA0;
	y *= n;
	y += &*ALPHA1;
	res += &y;
	res += &*ALPHA2;
	res *= &y;
	res += &*ALPHA3;
	res >>= 9;
	res / 3u8
}

pub fn expand_qrt2(input: Qrt2) -> Qrt2 {
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