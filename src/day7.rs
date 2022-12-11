#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn string_to_id (string: &str) -> u64 {
    string.chars()
    .map(|c|((c as u64).to_string())
    .collect::<String>()
    .parse::<u64>().unwrap()
}


pub fn day_7() {

    let file = File::open("2022_7.txt")
        .unwrap();
    BufReader::new(file)
        .lines();        
}
