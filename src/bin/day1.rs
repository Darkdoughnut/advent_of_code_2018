use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
		let result = 0; 

		let filename = "input";
		let file = File::open(filename);
	
		for line in BufReader::new(file).lines() {
        let char_vec:Vec<char> = line.chars().collect();
    }

}