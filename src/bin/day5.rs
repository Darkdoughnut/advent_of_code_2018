use std::fs::File;
use std::io::Read;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

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
        let mut react = false;
        {
            let end = reaction_vector.last().clone();
            match end {
                Some(x) => {
                    react = is_reaction_pair(*x, c);
                }
                None => {}
            }
        }
        if react {
            reaction_vector.pop();
        } else {
            reaction_vector.push(c);
        }
    }
    println!("Length after reaction: {}", reaction_vector.len());
    println!("----- Part 2 -----");
    let mut best_reaction_size = reaction_vector.len();
    for letter in &ASCII_LOWER {
        let mut vec_copy: Vec<char> = Vec::new();
        for c in reaction_vector.clone() {
            if c.to_lowercase().to_string() == letter.to_string() {
                continue;
            }
            let mut react = false;
            {
                let last_elem = vec_copy.last();
                match last_elem {
                    Some(x) => {
                        react = is_reaction_pair(*x, c);
                    }
                    None => {}
                }
            }
            if react {
                vec_copy.pop();
            } else {
                vec_copy.push(c);
            }
        }
        if vec_copy.len() < best_reaction_size {
            best_reaction_size = vec_copy.len();
        }
    }
    println!("Smallest reaction size: {}", best_reaction_size)
}

// #[test]
