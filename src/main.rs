use std::collections::HashMap;
use std::io::{self,BufRead};

fn main() {
    let mut map = HashMap::new();
    let istream = io::stdin();
    for word in istream.lock().lines() {
        for letter in word.unwrap().chars() {
            let store = &mut map;
            let count = *store.get(&letter).unwrap_or(&0);

            (*store).insert(letter,count+1); //
        }
    }

    for (letter,count) in map {
        println!("{},{}",count,letter);
    }
}
