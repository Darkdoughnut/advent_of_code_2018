use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line_to_point(line: String) -> (i32, i32) {
    // e.g.  249, 189
    let values_str = line.split(", ").collect::<Vec<&str>>();
    let first = values_str[0].parse::<i32>().unwrap();
    let second = values_str[1].parse::<i32>().unwrap();
    (first, second)
}

fn in_range(curr_point: (i32, i32), max_point: (i32, i32)) -> bool {
    if curr_point.0 < 0 {
        return false;
    }
    if curr_point.1 < 0 {
        return false;
    }
    if curr_point.0 >= max_point.0 || curr_point.1 >= max_point.1 {
        return false;
    }
    true
}

fn main() {
    println!("----- Part 1 -----");
    let file = File::open("input/day6").expect("Unable to open file.");
    let reader = BufReader::new(file);
    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut max_point: (i32, i32) = (0, 0);
    for line in reader.lines() {
        let line = line.unwrap();
        let curr_point = parse_line_to_point(line);
        if curr_point.0 > max_point.0 {
            max_point.0 = curr_point.0;
        }
        if curr_point.1 > max_point.1 {
            max_point.1 = curr_point.1;
        }
        points.push(curr_point);
    }
    let max_point = (max_point.0 + 1, max_point.1 + 1);

    // Insert points into array
    // Every point on the 2d arr is a pair (<point idx>, manhattan dist)
    // point idx is -2 if multiple points share the point
    // point idx is -1 if uninitialized
    // manhattan dist is -1 if uninitialized
    let mut map = vec![vec![(-1 as i32, -1 as i32); max_point.1 as usize]; max_point.0 as usize];
    let mut point_idx = 0;
    for point in &points {
        map[point.0 as usize][point.1 as usize] = (point_idx, 0);

        let mut manhattan_dist: i32 = 1;
        // Insert point in map for dist 0
        loop {
            let mut reach_limit = true;
            // Insert neg x, pos y diagonal
            for i in 0..manhattan_dist + 1 {
                let curr_x = point.0 - (i as i32);
                let curr_y = point.1 + (manhattan_dist as i32) - (i as i32);
                if in_range((curr_x, curr_y), max_point) {
                    // Adding point to map
                    if map[curr_x as usize][curr_y as usize].1 == -1 {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    } else if map[curr_x as usize][curr_y as usize].1 == manhattan_dist {
                        if map[curr_x as usize][curr_y as usize].0 != point_idx {
                            // reach_limit = false;
                            map[curr_x as usize][curr_y as usize].0 = -2;
                        }
                    } else if map[curr_x as usize][curr_y as usize].1 > manhattan_dist {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    }
                }
            }

            // Insert pos x, pos y diagonal
            for i in 0..manhattan_dist + 1 {
                let curr_x = point.0 + (i as i32);
                let curr_y = point.1 + (manhattan_dist as i32) - (i as i32);
                if in_range((curr_x, curr_y), max_point) {
                    // Adding point to map
                    if map[curr_x as usize][curr_y as usize].1 == -1 {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    } else if map[curr_x as usize][curr_y as usize].1 == manhattan_dist {
                        if map[curr_x as usize][curr_y as usize].0 != point_idx {
                            // reach_limit = false;
                            map[curr_x as usize][curr_y as usize].0 = -2;
                        }
                    } else if map[curr_x as usize][curr_y as usize].1 > manhattan_dist {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    }
                }
            }

            // Insert for neg x, neg y diagonal
            for i in 0..manhattan_dist + 1 {
                let curr_x = point.0 - (i as i32);
                let curr_y = point.1 - ((manhattan_dist as i32) - (i as i32));
                if in_range((curr_x, curr_y), max_point) {
                    // Adding point to map
                    if map[curr_x as usize][curr_y as usize].1 == -1 {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    } else if map[curr_x as usize][curr_y as usize].1 == manhattan_dist {
                        if map[curr_x as usize][curr_y as usize].0 != point_idx {
                            // reach_limit = false;
                            map[curr_x as usize][curr_y as usize].0 = -2;
                        }
                    } else if map[curr_x as usize][curr_y as usize].1 > manhattan_dist {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    }
                }
            }

            // Insert for pos x, neg y diagonal
            for i in 0..manhattan_dist + 1 {
                let curr_x = point.0 + (i as i32);
                let curr_y = point.1 - ((manhattan_dist as i32) - (i as i32));
                if in_range((curr_x, curr_y), max_point) {
                    // Adding point to map
                    if map[curr_x as usize][curr_y as usize].1 == -1 {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    } else if map[curr_x as usize][curr_y as usize].1 == manhattan_dist {
                        if map[curr_x as usize][curr_y as usize].0 != point_idx {
                            // reach_limit = false;
                            map[curr_x as usize][curr_y as usize].0 = -2;
                        }
                    } else if map[curr_x as usize][curr_y as usize].1 > manhattan_dist {
                        reach_limit = false;
                        map[curr_x as usize][curr_y as usize] = (point_idx, manhattan_dist);
                    }
                }
            }

            if reach_limit {
                break;
            } else {
                // Next loop add to manhattan distance
                manhattan_dist += 1;
            }
        }

        point_idx += 1;
    }

    let mut total_area_by_id: HashMap<i32, u32> = HashMap::new();
    for i in 0..points.len() - 1 {
        total_area_by_id.insert(i as i32, 0);
    }
    // Iterate through map border parallel to x
    for x in 0..max_point.0 - 1 {
        let bottom_line_id = map[x as usize][0].0;
        total_area_by_id.remove(&bottom_line_id);

        // repeat for top line
        let top_line_id = map[x as usize][(max_point.1 - 1) as usize].0;
        total_area_by_id.remove(&top_line_id);
    }
    // Iterate over border parallel to y
    for y in 0..max_point.1 - 1 {
        let left_line_id = map[0][y as usize].0;
        total_area_by_id.remove(&left_line_id);

        // repeat for top line
        let right_line_id = map[(max_point.0 - 1) as usize][y as usize].0;
        total_area_by_id.remove(&right_line_id);
    }

    for x in 0..max_point.0 {
        for y in 0..max_point.1 {
            let curr_idx = map[x as usize][y as usize].0;
            if let Some(id) = total_area_by_id.get_mut(&(curr_idx as i32)) {
                *id += 1;
            }
        }
    }
    // Code to print out gride

    // for x in 0..max_point.0 {
    //     for y in 0..max_point.1 {
    //         let curr_idx = map[x as usize][y as usize].0;
    //         let curr_dist = map[x as usize][y as usize].1;
    //         print!("{} ", format!("{:02}", curr_idx).to_string());
    //     }
    //     print!("\n");
    // }

    // Get largest area
    let mut best_area = 0;
    for (id, area) in total_area_by_id {
        println!("Curr id/area: {}/{}", id, area);
        if area > best_area {
            best_area = area;
        }
    }
    println!("Best area: {}", best_area);

    println!("----- Part 2 -----");
}

#[test]
fn test_parse_line_to_point() {
    let test_str = "249, 189".to_string();
    assert_eq!((249, 189), parse_line_to_point(test_str));
}
