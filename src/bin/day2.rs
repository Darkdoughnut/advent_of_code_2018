extern crate radix_trie;

// use radix_trie::{Trie, TrieCommon};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

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
    println!("2 letters {}", count_two_letters);
    println!("3 letters {}", count_three_letters);
    println!("Result: {}", result);

    // Part 2
    let file = File::open("input/day2").expect("Unable to open");
    let reader = BufReader::new(file);
}

fn scan_word_for_n(word: &str, count: usize) -> i64 {
    let mut letter_map: HashMap<char, usize> = HashMap::new();

    for (_letter, letter_count) in letter_map {
        if letter_count == count {
            return 1;
        }
    }
    return 0;
}

fn find_similar_words<B: BufRead>(reader: B) -> String {
    // When inserting a word in the trie, a \n character
    // will signify the end of the word
    let mut word_bank: HashSet<String> = HashSet::new();
    return "Hi".to_string();
}

// TODO: FIGURE OUT TRIE IMPL
// fn find_similar_words<B: BufRead>(reader: B) -> String {
//     // When inserting a word in the trie, a \n character
//     // will signify the end of the word
//     let mut trie = Trie::new();
//     for line in reader.lines() {
//         let line = line.expect("Could not get line");
//         // Check for a similar word in the radix tree
//     }
//     return "Hi".to_string();
// }

// fn check_trie_contains_word<T: Trie>(trie: T, char_itr: Chars, diff_found: bool) -> bool {
//     // TODO: Handle empty string case
//     if !diff_found {

//     } else {
//         let get_option = trie.get_mut(*char_itr);
//         match get_option {
//             Some(x) => return check_trie_contains_word(x, char_itr, diff_found),
//             None => return false,
//         }
//     }
// }

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
