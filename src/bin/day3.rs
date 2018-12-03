use std::fs::File;
use std::io::{BufRead, BufReader};

struct Position {
    left_margin: u32,
    top_margin: u32,
    width: u32,
    height: u32,
    id: u32,
}

fn main() {
    println!("----- Part 1 -----");
    let file = File::open("input/day3").expect("Unable to open");
    let reader = BufReader::new(file);
    let mut positions: Vec<Position> = Vec::new();
    // Read in values
    for line in reader.lines() {
        let mut line = line.expect("Err getting line");
        positions.push(convert_line_to_position(line));
    }
    // Calculate max height and width
    let mut max_width = 0;
    let mut max_height = 0;
    for pos in &positions {
        if (pos.left_margin + pos.width) > max_width {
            max_width = pos.left_margin + pos.width;
        }
        if (pos.top_margin + pos.height) > max_height {
            max_height = pos.top_margin + pos.height;
        }
    }

    let mut fabric = vec![vec![0u32; max_width as usize]; max_height as usize];
    for pos in &positions {
        // Increment arr
        let start_i = pos.top_margin;
        let start_j = pos.left_margin;
        for x in 0..pos.height {
            for y in 0..pos.width {
                fabric[(start_i + x) as usize][(start_j + y) as usize] += 1;
            }
        }
    }
    let mut overlap_claims = 0;
    for x in 0..max_height {
        for y in 0..max_width {
            if fabric[x as usize][y as usize] > 1 {
                overlap_claims += 1;
            }
        }
    }
    println!("Total overlapping fabric in sq. in: {}", overlap_claims);

    println!("----- Part 2 -----");
    // Find a position where every value in the fabric is 1
    for pos in &positions {
        let start_i = pos.top_margin;
        let start_j = pos.left_margin;
        let mut invalid_claim = false;
        for x in 0..pos.height {
            for y in 0..pos.width {
                if fabric[(start_i + x) as usize][(start_j + y) as usize] != 1 {
                    invalid_claim = true;
                    break;
                }
            }
            if invalid_claim {
                break;
            }
        }
        if !invalid_claim {
            // Found non-overlapping claim
            println!("Found valid non-overlapping claim!");
            println!("Claim id: {}", pos.id);
        }
    }
}

fn convert_line_to_position(line: String) -> Position {
    // #123 @ 3,2: 5x4
    let split = line.split_whitespace();
    // ['#123, '@', '3,2:', '5x4']
    let res = split.collect::<Vec<&str>>();
    // Get id
    let id = res[0].trim_matches('#').parse::<u32>().unwrap();

    let mut margins_itr = res[2].trim_matches(':').split(',');
    let left_margin = margins_itr.next().unwrap().parse::<u32>().unwrap();
    let top_margin = margins_itr.next().unwrap().parse::<u32>().unwrap();

    let mut dim_itr = res[3].split('x');
    let width = dim_itr.next().unwrap().parse::<u32>().unwrap();
    let height = dim_itr.next().unwrap().parse::<u32>().unwrap();

    return Position {
        left_margin,
        top_margin,
        width,
        height,
        id,
    };
}

#[test]
fn test_convert_line_to_position() {
    let test_line = "#123 @ 3,2: 5x4".to_string();
    let test_pos = Position {
        left_margin: 3,
        top_margin: 2,
        width: 5,
        height: 4,
        id: 123,
    };
    let res = convert_line_to_position(test_line);
    assert_eq!(test_pos.left_margin, res.left_margin);
    assert_eq!(test_pos.top_margin, res.top_margin);
    assert_eq!(test_pos.width, res.width);
    assert_eq!(test_pos.height, res.height);
    assert_eq!(test_pos.id, res.id);
}
