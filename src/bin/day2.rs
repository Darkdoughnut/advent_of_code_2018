use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Part 1
    let mut count_three_letters: i64 = 0;
    let mut count_two_letters: i64 = 0;

    let file = File::open("input/day2").expect("Unable to open");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("error getting line");
        count_two_letters += scan_word_for_n(&line, 2);
        count_three_letters += scan_word_for_n(&line, 3);
    }
    let result = count_two_letters * count_three_letters;
    println!("{}", result);

    // Part 2
}

fn scan_word_for_n(word: &str, count: usize) -> i64 {
    let mut letter_map: HashMap<char, usize> = HashMap::new();
    for c in word.chars() {
        *(letter_map.entry(c).or_insert(0)) += 1;
    }
    for (_letter, letter_count) in letter_map {
        if letter_count == count {
            return 1;
        }
    }
    return 0;
}

#[test]
fn test_scan_word_for_n() {
    assert_eq!(scan_word_for_n("abcdef", 2), 0);

    assert_eq!(scan_word_for_n("bababc", 2), 1);
    assert_eq!(scan_word_for_n("bababc", 3), 1);

    assert_eq!(scan_word_for_n("abbcde", 2), 1);
    assert_eq!(scan_word_for_n("abbcde", 3), 0);

    assert_eq!(scan_word_for_n("abcccd", 2), 0);
    assert_eq!(scan_word_for_n("abcccd", 3), 1);

    assert_eq!(scan_word_for_n("abcdee", 2), 1);
    assert_eq!(scan_word_for_n("abcdee", 3), 0);

    assert_eq!(scan_word_for_n("ababab", 2), 0);
    assert_eq!(scan_word_for_n("ababab", 3), 1);
}
