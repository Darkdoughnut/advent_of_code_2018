use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
	// Part 1
	let file = File::open("input/day1")?;
	let reader = BufReader::new(file);
	let result = sum_lines(reader)?;
	println!("{}", result);

	// Part 2
	let file = File::open("input/day1")?;
	let reader = BufReader::new(file);
	let result = first_rep_freq(reader)?;
	println!("{}", result);

	Ok(())
}

fn sum_lines<B: BufRead>(reader: B) -> io::Result<i64> {
	let mut sum = 0;
	for line in reader.lines() {
		sum += line?.parse::<i64>().unwrap();
	}
	Ok(sum)
}

fn first_rep_freq<B: BufRead>(reader: B) -> io::Result<i64> {
	let mut changes = Vec::new();
	for line in reader.lines() {
		changes.push(line?.parse::<i64>().unwrap());
	}

	let mut freq = 0;
	let mut visited_freq = HashSet::new();
	loop {
		for change in &changes {
			if visited_freq.contains(&freq) {
				return Ok(freq);
			}
			visited_freq.insert(freq);
			freq += change;
		}
	}
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

#[test]
fn test_first_rep_req() {
	assert_eq!(first_rep_freq(BufReader::new(&b"+0"[..])).unwrap(), 0);
	assert_eq!(first_rep_freq(BufReader::new(&b"+1\n-1\n"[..])).unwrap(), 0);
	assert_eq!(
		first_rep_freq(BufReader::new(&b"+7\n+7\n-2\n-7\n-4"[..])).unwrap(),
		14
	);
	assert_eq!(
		first_rep_freq(BufReader::new(&b"-6\n+3\n+8\n+5\n-6"[..])).unwrap(),
		5
	);
}
