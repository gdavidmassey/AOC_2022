#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn overlap<T>(pairs: &[T]) -> u32
where
    T: PartialOrd + Debug,
{
    // println!("{:?}", pairs);
    if (pairs[2] >= pairs[0] && pairs[3] <= pairs[1])
        || (pairs[0] >= pairs[2] && pairs[1] <= pairs[3])
    {
        return 1;
    }
    0
}

fn partial_overlap(pairs: &[u32]) -> u32
//where
//    T: PartialOrd + Debug,
{
    if (pairs[2]..(pairs[3] + 1)).contains(&pairs[0])
        || (pairs[2]..(pairs[3] + 1)).contains(&pairs[1])
        || (pairs[0]..(pairs[1] + 1)).contains(&pairs[2])
        || (pairs[0]..(pairs[1] + 1)).contains(&pairs[3])
    //if (pairs[0] >= pairs[2] && pairs[0] <= pairs[3])
    //    || (pairs[1] >= pairs[2] && pairs[1] <= pairs[3])
    //    || (pairs[2] >= pairs[0] && pairs[2] <= pairs[1])
    //    || (pairs[3] >= pairs[0] && pairs[3] <= pairs[1])
    {
        return 1;
    }
    0
}

pub fn day_4() {
    let x = 5;
    let f = File::open("2022_4.txt").unwrap();
    let buff = BufReader::new(f);
    let mut pairs = Vec::new();
    buff.lines().for_each(|x| pairs.push(x.unwrap()));
    let mut split_pairs = Vec::new();
    pairs.iter().for_each(|x| {
        split_pairs.push(
            x.split(|x| x == ',' || x == '-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    });

    split_pairs.iter().for_each(|x| println!("{:?}", x));

    let count_pairs = split_pairs.iter().fold(0, |acc, x| acc + overlap(x));
    let count_partial_overlap = split_pairs
        .iter()
        .fold(0, |acc, x| acc + partial_overlap(x));
    let five = 5u32;
    println!("{:?}", (1..five).contains(&5u32));
    println!("{}", count_pairs);
    println!("{}", count_partial_overlap);
}
