use rug::Integer;
use mosers_powers::calc_to_from;

fn main() {
	calc_to_from(1u32, 100000u32, Integer::from(1u32));
}