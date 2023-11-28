use rug::Integer;
use mosers_powers::calc_to_from;
use mosers_powers::Qrt2;
use std::fs;
use std::cmp;

fn main() {
	let from_checkpoint: bool = true;
	let write: bool = true;
	
	let min: u32 =  500000;
	let max: u32 = 1000000;
	let by:  u32 =  100000;
	let mut n: Integer = Integer::from(1u8);
	let mut qrt2 = Qrt2 {num: Integer::from(2435), basepow: 11u32};
	
	if from_checkpoint {
		let cp = read_checkpoint(x);
		n = cp.0;
		qrt2 = cp.1;
	}
	
	let mut x = min;
	while min < max {
		let tmp_max: u32 = cmp::min(max, min+by);
		let result: (Integer, Qrt2, u128) = calc_to_from(min, tmp_max, n, qrt2);

		println!("Milestone: x={0} (last iter took {1}ms)", &tmp_max, result.2);

		if write {
			write_checkpoint(tmp_max, result.0, result.1);
		}

		x += by;
	}

}

fn write_checkpoint(x: u32, n: Integer, qrt: Qrt2) {
	let data = format!("{}\n{}\n{}\n{}", x, n, qrt.num, qrt.basepow);
	fs::write(format!("{}.cp", x), data).expect("Can't write file");
}

fn read_checkpoint(x: u32) -> (Integer, Qrt2) {
	let data = fs::read_to_string(format!("{}.cp", x)).expect(&format!("Can't find checkpoint {}", x));
	let mut lines = data.split("\n");
	let x_read = lines.next().expect("Checkpoiont file is missing the first line!").parse::<u32>().unwrap();
	if x_read != x {
		println!("Checkpoint file doesn't match!");
		return;
	}
	let n = Integer::from_str_radix(lines.next().expect("Checkpoiont file is missing the second line!"), 10).ok().expect("Parse error on n!");
	let qrt2_num = Integer::from_str_radix(lines.next().expect("Checkpoiont file is missing the third line!"), 10).ok().expect("Parse error on num!");
	let qrt2_base = lines.next().expect("Checkpoiont file is missing the fourth line!").parse::<u32>().unwrap();

	return (n, Qrt2 {num: qrt2_num, basepow: qrt2_base})
}