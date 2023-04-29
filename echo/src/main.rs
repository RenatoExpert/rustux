use std::env;

fn main() {
	let arguments: Vec<String> = env::args().collect();
	let mut i: i32 = 0;
	for argument in arguments {
		/*
		if i > 0 {
			if argument.chars().next().unwrap() == '-' {
				if argument.contains("a") {show_hidden = true};
				if argument.contains("l") {long_mode = true};
			} else {
				path = argument;
			}
		}
		*/
		if i>0 {
			print!("{} ", argument);
		}
		i+=1;
	}
	print!("\n");
}
