use rug::Integer;
use mosers_powers::calc_to_from;
use mosers_powers::Qrt2;
use std::fs;

fn main() {
	let from_checkpoint: bool = true;
	let write_checkpoint: bool = true;
	
	let min: u32 = 10000;
	let max: u32 = 100000;
	let mut n: Integer = Integer::from(1u8);
	let mut qrt2_num: Integer = Integer::from(2435);
	let mut qrt2_base: u32 = 11;
	
	if from_checkpoint {
		let data = fs::read_to_string(format!("{}.cp", min)).expect(&format!("Can't find checkpoint {}", min));
		let mut lines = data.split("\n");
		let x = lines.next().expect("Checkpoiont file is missing the first line!").parse::<u32>().unwrap();
		if x != min {
			println!("Checkpoint file doesn't match!");
			return;
		}
		n = Integer::from_str_radix(lines.next().expect("Checkpoiont file is missing the second line!"), 10).ok().expect("Parse error on n!");
		qrt2_num = Integer::from_str_radix(lines.next().expect("Checkpoiont file is missing the third line!"), 10).ok().expect("Parse error on num!");
		qrt2_base = lines.next().expect("Checkpoiont file is missing the fourth line!").parse::<u32>().unwrap();
	}
	
	let result: (Integer, Qrt2) = calc_to_from(min, max, n, Qrt2 {num: qrt2_num, basepow: qrt2_base});
	
	if write_checkpoint {
		let data = format!("{}\n{}\n{}\n{}", max, result.0, result.1.num, result.1.basepow);
		fs::write(format!("{}.cp", max), data).expect("Can't write file");
	}
}