use rug::Integer;
use std::fs;

use crate::Qrt2;

pub fn write_checkpoint(x: &u32, n: &Integer, qrt: &Qrt2) {
	let data = format!("{}\n{}\n{}\n{}", x, n, qrt.num, qrt.basepow);
	fs::write(format!("cps/{}.cp", x), data).expect("Can't write file");
}

pub fn read_checkpoint(x: u32) -> (Integer, Qrt2) {
	let data = fs::read_to_string(format!("cps/{}.cp", x)).expect(&format!("Can't find checkpoint {}", x));
	let mut lines = data.split("\n");
	let x_read = lines.next().expect("Checkpoint file is missing the first line!").parse::<u32>().unwrap();
	if x_read != x {
		panic!("Checkpoint file doesn't match!");
	}
	let n = Integer::from_str_radix(lines.next().expect("Checkpoiont file is missing the second line!"), 10).ok().expect("Parse error on n!");
	let qrt2_num = Integer::from_str_radix(lines.next().expect("Checkpoiont file is missing the third line!"), 10).ok().expect("Parse error on num!");
	let qrt2_base = lines.next().expect("Checkpoiont file is missing the fourth line!").parse::<u32>().unwrap();

	return (n, Qrt2 {num: qrt2_num, basepow: qrt2_base})
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