use num_bigint::BigUint;
use mosers_powers::calc_to_from;

fn main() {
	calc_to_from(2u32, 1000000u32, BigUint::from(3u32));
}