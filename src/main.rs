use clap::{Arg, ArgAction, Command};
use rug::Integer;
use std::cmp;
use std::time::Instant;

use mosers_powers::*;
use mosers_powers::io::*;

fn main() {
	
	let matches = Command::new("Moser's Powers")
		.version("0.3.0")
		.arg(Arg::new("min")
			.index(1)
			.required(true)
			.value_parser(clap::value_parser!(u32))
			.help("Minimum x-value (power of 2)"))
		.arg(Arg::new("max")
			.index(2)
			.required(true)
			.value_parser(clap::value_parser!(u32))
			.help("Maximum x-value"))
		.arg(Arg::new("by")
			.index(3)
			.required(true)
			.value_parser(clap::value_parser!(u32))
			.help("x-value steps for messages, checkpoints"))
		.arg(Arg::new("manual")
			.long("manual")
			.short('m')
			.action(ArgAction::SetTrue)
			.requires("n")
			.requires("qrt2")
			.help("Set n and qrt(2) values manually instead of using checkpoint file"))
		.arg(Arg::new("n")
			.short('n')
			.long("n_start")
			.num_args(1)
			.help("Starting n-value"))
		.arg(Arg::new("qrt2")
			.short('q')
			.long("qrt")
			.num_args(2)
			.value_names(["num", "denom"])
			.help("qrt(2) approximation numerator and denominator (log 2)"))
		.arg(Arg::new("disable_cp")
			.long("disable_cp")
			.short('d')
			.action(ArgAction::SetTrue)
			.help("Disable writing checkpoint files"))
		.arg(Arg::new("message_output")
			.long("message_output")
			.short('o')
			.action(ArgAction::SetTrue)
			.help("Enable writing message files"))
		.get_matches();
	
	let from_checkpoint: bool = !matches.get_flag("manual");
	let write: bool = !matches.get_flag("disable_cp");
	let message_file: bool = matches.get_flag("message_output");
	let min: u32 = *matches.get_one("min").expect("min is required!");
	let max: u32 = *matches.get_one("max").expect("max is required!");
	let by:  u32 = *matches.get_one("by").expect("by is required!");
	
	let mut n: Integer;
	let mut qrt2: Qrt2;
	
	if from_checkpoint {
		let cp = read_checkpoint(min);
		n = cp.0;
		qrt2 = cp.1;
	} else {
		n = Integer::from_str_radix(matches.get_one::<String>("n").expect("n is required in manual mode!"), 10).expect("Failed to parse n as an integer!");
		let num: Integer = Integer::from_str_radix(matches.get_one::<String>("num").expect("qrt(2) numerator is required in manual mode!"), 10).expect("Failed to parse qrt(2) numerator as an integer!");
		let den: u32 = *matches.get_one("den").expect("qrt(2) denominator is required in manual mode!");
		qrt2 = Qrt2 {num: num, basepow: den};
	}
	
	let mut x = min;
	while x < max {
		let tmp_max: u32 = cmp::min(max, x+by);
		let result: (u32, Integer, Qrt2, u128) = calc_to_from(x, tmp_max, n.clone(), qrt2, message_file);

		println!("Milestone: x={0} (last iter took {1}ms)", &tmp_max, result.3);
		
		if message_file {
			message_cp(&result.0);
		}
		
		if write {
			write_checkpoint(&result.0, &result.1, &result.2);
		}
		
		x = result.0;
		n = result.1;
		qrt2 = result.2;
	}

}

fn calc_to_from(min: u32, max: u32, nstart: Integer, qrt_start: Qrt2, file: bool) -> (u32, Integer, Qrt2, u128) {
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