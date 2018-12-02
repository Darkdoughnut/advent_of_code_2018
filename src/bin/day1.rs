use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("input/day1")?;
	let reader = BufReader::new(file);
	let result = sum_lines(reader)?;
	println!("{}", result);

	Ok(())
}

fn sum_lines<B: BufRead>(reader: B) -> io::Result<i64> {
	let mut sum: i64 = 0;
	for line in reader.lines() {
		sum += line?.parse::<i64>().unwrap();
	}
	Ok(sum)
}

#[test]
fn test_sum_lines() {
	assert_eq!(sum_lines(BufReader::new(&b"+1\n+1\n+1\n"[..])).unwrap(), 3);
	assert_eq!(sum_lines(BufReader::new(&b"+1\n+1\n-2\n"[..])).unwrap(), 0);
	assert_eq!(sum_lines(BufReader::new(&b"-1\n-2\n-3\n"[..])).unwrap(), -6);
	assert_eq!(
		sum_lines(BufReader::new(&b"-1\n+0\n-0\n+2"[..])).unwrap(),
		1
	);
}
