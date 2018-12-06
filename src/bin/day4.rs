use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Default, Eq, PartialEq, PartialOrd, Ord)]
struct Event(Date, String);

#[derive(Clone)]
struct Guard {
    total_sleep: u32,
    sleep_count: Vec<u32>,
}

impl Guard {
    fn up_sleep_count(&mut self, index: u8) {
        // ^^^ Here
        self.sleep_count[index as usize] += 1;
    }
}

fn string_to_date(date_str: String) -> Date {
    // Input [1518-04-28 00:01]
    let date_str = date_str.trim_matches('[').trim_matches(']');
    let res = date_str.split(' ').collect::<Vec<&str>>();
    let mut date_itr = res[0].split('-');
    let year = date_itr.next().unwrap().parse::<u16>().unwrap();
    let month = date_itr.next().unwrap().parse::<u8>().unwrap();
    let day = date_itr.next().unwrap().parse::<u8>().unwrap();

    let mut time_itr = res[1].split(':');
    let hour = time_itr.next().unwrap().parse::<u8>().unwrap();
    let minute = time_itr.next().unwrap().parse::<u8>().unwrap();

    return Date {
        year: year,
        month: month,
        day: day,
        hour: hour,
        minute: minute,
    };
}

fn parse_guard_num(msg: String) -> u32 {
    let split = msg.split(' ');
    let guard_id_str = split.collect::<Vec<&str>>()[1].trim_matches('#');
    return guard_id_str.parse::<u32>().unwrap();
}

fn main() {
    println!("----- Part 1 -----");
    let file = File::open("./input/day4").expect("Unable to open");
    let reader = BufReader::new(file);

    let mut timestamps: Vec<Event> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error getting string.");
        let mut line_itr = line.split("] ");
        let curr_date = string_to_date(line_itr.next().unwrap().to_string());
        let message = line_itr.next().unwrap();
        timestamps.push(Event(curr_date, message.to_string()));
    }
    timestamps.sort();
    assert!(timestamps.len() != 0);
    let mut guard_info: HashMap<u32, Guard> = HashMap::new();
    let mut timestamp_itr = timestamps.iter();

    let mut curr_date: Date;
    let mut curr_msg: String;
    let mut guard_num: u32 = 0;
    let mut time_asleep: u8 = 0;
    loop {
        // Get guard num
        let curr_event_itr = timestamp_itr.next();
        match curr_event_itr {
            Some(x) => {
                curr_date = x.0.clone();
                curr_msg = x.1.clone();
            }
            None => break,
        }
        if curr_msg == "falls asleep" {
            time_asleep = curr_date.minute;
        } else if curr_msg == "wakes up" {
            guard_info.get_mut(&guard_num).unwrap().total_sleep +=
                (curr_date.minute - time_asleep) as u32;

            let guard_mins = guard_info.get_mut(&guard_num).unwrap();
            for x in time_asleep..curr_date.minute {
                guard_mins.up_sleep_count(x);
            }
        } else {
            guard_num = parse_guard_num(curr_msg);
            // Check if guard is in guard info
            if !guard_info.contains_key(&guard_num) {
                guard_info.insert(
                    guard_num,
                    Guard {
                        total_sleep: 0,
                        sleep_count: vec![0u32; 60],
                    },
                );
            }
        }
    }
    let mut best_guard_num = 0;
    let mut most_sleep = 0;
    let mut best_guard_minute = 0;
    let mut most_guard_minute_instance = 0;
    for (guard_id, curr_guard_info) in guard_info.clone() {
        if curr_guard_info.total_sleep > most_sleep {
            best_guard_num = guard_id;
            most_sleep = curr_guard_info.total_sleep;
        }
    }
    let best_total_sleep = &guard_info.get(&best_guard_num).unwrap().sleep_count;
    for x in 0..59 {
        if best_total_sleep[x] > most_guard_minute_instance {
            best_guard_minute = x;
            most_guard_minute_instance = best_total_sleep[x];
        }
    }
    println!("Guard id: {}", best_guard_num);
    println!("Best minute: {}", best_guard_minute);
    println!("Solution: {}", (best_guard_minute as u32) * best_guard_num);

    println!("----- Part 2 -----");
    let mut best_guard_num = 0;
    let mut best_guard_minute = 0;
    let mut most_guard_minute_instance = 0;
    for (guard_id, curr_guard_info) in guard_info.clone() {
        let best_total_sleep = &curr_guard_info.sleep_count;
        for x in 0..59 {
            if best_total_sleep[x] > most_guard_minute_instance {
                best_guard_num = guard_id;
                best_guard_minute = x;
                most_guard_minute_instance = best_total_sleep[x];
            }
        }
    }
    println!("Guard id: {}", best_guard_num);
    println!("Best minute: {}", best_guard_minute);
    println!("Solution: {}", (best_guard_minute as u32) * best_guard_num);
}

#[test]
fn test_string_to_date() -> () {}
