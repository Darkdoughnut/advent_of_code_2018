use std::fs::File;
use std::io::Read;

fn is_reaction_pair(a: char, b: char) -> bool {
    assert!(a.is_lowercase() || a.is_uppercase());
    assert!(b.is_lowercase() || b.is_uppercase());

    if a.is_lowercase() != b.is_lowercase() {
        // xor
        if a.to_lowercase().to_string() == b.to_lowercase().to_string() {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn main() {
    println!("----- Part 1 -----");
    let mut file = File::open("input/day5").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let mut reaction_vector: Vec<char> = Vec::new();
    for c in contents.chars() {
        let end = reaction_vector.last().clone();
        match end {
            Some(x) => {
                if is_reaction_pair(*x, c) {
                    reaction_vector.pop();
                }
            }
            None => reaction_vector.push(c),
        }
    }
    println!("Length after reaction: {}", reaction_vector.len());
    println!("----- Part 2 -----");
}

// #[test]
