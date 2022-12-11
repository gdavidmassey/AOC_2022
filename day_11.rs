#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub fn day_7() {
    let file = File::open("2022_11.txt").unwrap();
    let mut instruction = BufReader::new(file).lines();

}
