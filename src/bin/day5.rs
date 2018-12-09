use std::io::{BufRead, BufReader};
use std::fs::File;

fn is_reaction_pair(a: char, b: char) -> bool {
    assert!(a.is_lowercase() || a.is_uppercase());
    assert!(b.is_lowercase() || b.is_uppercase());

    if (a.is_lowercase() != b.is_lowercase()){ // xor
        if (a.to_lowercase() == b.to_lowercase()){
            return true;
        }else{
            return false;
        }
    }else{
        return false;
    }
}

fn main() {
    println!("----- Part 1 -----");
    let file = File::open("input/day5");
    let reader = BufReader::new(file);
    let mut reaction_vector: Vec<char> = Vec::new();
    for c in reader.chars() {
        let end = reaction_vector.last();
        match end {
            Some(x) => {
                if is_reaction_pair(x, c){
                    reaction_vector.pop();
                }
            },
            None => reaction_vector.push(c);
        }

    }
    println!("Length after reaction: {}", reaction_vector.len());
    println!("----- Part 2 -----");
}

#[test]

