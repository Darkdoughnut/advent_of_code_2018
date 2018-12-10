use std::fs::File;
use std::io::{BufRead, BufReader};

// Maps the capital letter to an index 0..25
fn get_edge_from_string(line: String) -> (u16, u16) {
    // Sample input
    // Step D must be finished before step P can begin.
    // D -> P
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let first = tokens[1].chars().nth(0).expect("Empty string");
    let second = tokens[7].chars().nth(0).expect("Empty string");
    return (first as u16 - 65, second as u16 - 65);
}

fn main() {
    println!("----- Part 1 -----");
    let file = File::open("input/day7").expect("Unable to open file.");
    let reader = BufReader::new(file);

    const ALPHABET_SIZE: usize = 26;
    let mut dag = vec![[false; ALPHABET_SIZE]; ALPHABET_SIZE];
    let mut node_edge_count = vec![-1 as i32; ALPHABET_SIZE];
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let edge: (u16, u16) = get_edge_from_string(line);
        dag[edge.0 as usize][edge.1 as usize] = true;
        if node_edge_count[edge.0 as usize] == -1 {
            node_edge_count[edge.0 as usize] = 0;
        }
        if node_edge_count[edge.1 as usize] == -1 {
            node_edge_count[edge.1 as usize] = 0;
        }
        // Add to edge count
        node_edge_count[edge.1 as usize] += 1;
    }

    let mut no_dep_queue: Vec<u16> = Vec::new();
    for node_idx in 0..node_edge_count.len() {
        if node_edge_count[node_idx] == 0 {
            // No dependencies
            no_dep_queue.push(node_idx as u16);
        }
    }

    while !no_dep_queue.is_empty() {
        no_dep_queue.sort();
        // Remove item from no_dep_queue
        let front = no_dep_queue.remove(0);
        print!("{}", (front as u8 + 65) as char);
        for i in 0..ALPHABET_SIZE {
            if dag[front as usize][i] {
                // There is an edge from front to i
                dag[front as usize][i] = false;
                node_edge_count[i] -= 1;
                if node_edge_count[i] == 0 {
                    no_dep_queue.push(i as u16);
                }
            }
        }
    }
    print!("\n");

    println!("----- Part 2 -----");
}

#[test]
fn test_parse_string() {
    let test_str = "Step Z must be finished before step B can begin.".to_string();
    assert_eq!(get_edge_from_string(test_str), (25 as u16, 1 as u16));
}
