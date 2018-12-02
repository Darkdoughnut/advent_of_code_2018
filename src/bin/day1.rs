use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
	let mut result = 0;
	let file = File::open("input/day1")?;

	for line in BufReader::new(file).lines() {
		let num: i64 = line?.parse().unwrap();
		result += num;
	}
	println!("{}", result);

	Ok(())
}
