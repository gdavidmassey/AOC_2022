#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

fn string_to_id(string: &str) -> u64 {
    string
        .chars()
        .map(|c| (c as u64).to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Tree {
    data: HashMap<u64, u64>,
    parent: HashMap<u64, u64>,
    child: HashMap<u64, HashSet<u64>>,
    nested_size: HashMap<u64, u64>,
    pointer: u64,
}

impl Tree {
    fn new(root_id: u64) -> Self {
        Self {
            data: HashMap::new(),
            parent: HashMap::new(),
            child: HashMap::new(),
            nested_size: HashMap::new(),
            pointer: root_id,
        }
    }

    fn add_subdir(&mut self, directory_id: u64, parent_id: u64) {
        let children = self.child.entry(parent_id).or_insert(HashSet::new());
        (*children).insert(directory_id);
        self.parent.insert(directory_id, parent_id);
        self.data.insert(directory_id, 0);
    }

    fn add_data(&mut self, directory_id: u64, file_size: u64) {
        let data = self.data.entry(directory_id).or_insert(0);
        *data += file_size;
    }

    fn get_nested_size(&mut self) {
        let passable = Rc::new(RefCell::new(self));
        fn recurse(directory_id: u64, tree: Rc<RefCell<&mut Tree>>) -> Option<u64> {
            let x = tree.get_mut();

            let mut size = *x.data.get(&directory_id).unwrap();
            for child in x.child.get(&directory_id)? {
                size += recurse(*child, tree).unwrap_or_else(|| 0);
            }
            Some(size)
        }
        let x = passable.clone();
        let x = x.get_mut();
        for folder in x.data.keys() {
            x.nested_size
                .insert(*folder, recurse(*folder, passable).unwrap());
        }
    }
}

pub fn day_7() {
    let file = File::open("2022_7_test.txt").unwrap();
    let mut instruction = BufReader::new(file).lines();

    println!("{}", string_to_id("a"));
    println!("{}", string_to_id("b"));
    println!("{}", string_to_id("ab"));

    let mut tree = Tree::new(string_to_id("/"));

    tree.add_subdir(0, 1);
    tree.add_subdir(1, 2);
    tree.add_subdir(1, 3);
    tree.add_data(3, 45);
    println!("{:?}", tree);

    let mut parse_command = |command: String| {
        let command = command.split(" ").collect::<Vec<&str>>();
        match command[0] {
            "$" => match command[1] {
                "cd" => match command[2] {
                    ".." => {
                        tree.pointer = *tree.parent.get(&(tree.pointer)).expect("u64 parent_id")
                    }
                    _ => tree.pointer = string_to_id(command[2]),
                },
                _ => (),
            },
            "dir" => tree.add_subdir(string_to_id(command[1]), tree.pointer),
            x => tree.add_data(
                tree.pointer,
                command[0].parse::<u64>().expect("u64 file size"),
            ),
        }
    };

    loop {
        let command = instruction.next();
        match command {
            None => break,
            Some(command) => match command {
                Ok(command) => parse_command(command),
                Err(_) => break,
            },
        }
    }
}
