use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn get_current_count(letter: &char, map: &HashMap<char, i32>) -> i32 {
    return *map.get(letter).unwrap_or(&0);
}

fn put_new_count(letter: char, count: i32, map: &mut HashMap<char, i32>) {
    map.insert(letter, count);
}

fn main() {
    let mut map = HashMap::new();
    let istream = io::stdin();
    for word in istream.lock().lines() {
        let mut used: HashSet<char> = HashSet::new();
        for letter in word.unwrap().chars() {
            if !used.contains(&letter) {
                let count = get_current_count(&letter, &map);
                put_new_count(letter, count + 1, &mut map);
            }
            used.insert(letter);
        }
    }

    for (letter, count) in map {
        println!("{},{}", count, letter);
    }
}
