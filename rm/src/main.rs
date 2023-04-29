use std::fs;
use std::env;

fn main() {
	//	Interpreting arguments
	let mut files:Vec<String> = Vec::new();
	let mut flags:Vec<char> = Vec::new();
	let args:Vec<String> = env::args().collect();
	let mut i:i32 = 0;
	for arg in args {
		if i==0 {
		} else if arg.chars().next().unwrap()=='-' {
			for char in arg.chars() {
				if char.is_alphabetic() {
					flags.push(char);
				}
			}
		} else {
			files.push(arg);
		}
		i+=1;
	}
	//	Instanciating flags
	let force = flags.contains(&'f');
	let recursive = flags.contains(&'r');
	let interactive = flags.contains(&'i');
	let directory = flags.contains(&'d');
	//	Deleting files
	for file in files {
		if f {
			fs::remove_file(file);
		}
	}
}
