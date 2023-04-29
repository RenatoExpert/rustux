use std::fs;
use std::env;

fn main() {
	let mut show_hidden: bool = false;
	let mut long_mode: bool = false;
	let mut path: String = ".".to_string();
	let mut i: i32 = 0;

	let arguments: Vec<String> = env::args().collect();
	for argument in arguments {
		if i > 0 {
			if argument.chars().next().unwrap() == '-' {
				if argument.contains("a") {show_hidden = true};
				if argument.contains("l") {long_mode = true};
			} else {
				path = argument;
			}
		}
		i+=1;
	}

	for file in fs::read_dir(path).unwrap() {
		let mut name = file.unwrap().path().display().to_string();
		if name.chars().next().unwrap() == '.' {
			name = name.replace("./", "");
		} else if name.chars().next().unwrap() == '/' {
			name = name.replace("/", "");
		}

		if name.chars().next().unwrap() != '.' || show_hidden {
			//let metadata = fs::metadata(name);
			//dbg!(metadata);
			print!("{}\t", name);
			//if long_mode {print!("\n")};
		} 
	}
	print!("\n");
}

