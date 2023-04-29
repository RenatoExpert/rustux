use std::fs;
use std::env;

fn main() {
	let mut i: i32 = 0;

	let arguments: Vec<String> = env::args().collect();
	if arguments.len() == 1 {
		println!("mkdir: missing operand");
		println!("Try 'mkdir --help' for more information.");
	} else {
		for argument in arguments {
		if i > 0 {
				if argument.chars().next().unwrap() == '-' {
				} else {
					fs::create_dir(argument);
				}
			}
			i+=1;
		}
	}
}
