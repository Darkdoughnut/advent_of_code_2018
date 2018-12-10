use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_edge_from_string(line: String) -> (char, char){
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let first = tokens[1].chars().nth(0).expect("Empty string");
    let second = tokens[7].chars().nth(0).expect("Empty string");
    return (first, second);
}

fn main() {
    println!("----- Part 1 -----");
    let file =  File::open("input/day7").expect("Unable to open file.");
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.expect("Error reading line");
        get_edge_from_string(line);
    }

    println!("----- Part 2 -----");
}

#[test]
fn test_parse_string() {
    let test_str = "Step Z must be finished before step B can begin.".to_string();
    assert_eq!(get_edge_from_string(test_str), ('Z', 'B'));
}
