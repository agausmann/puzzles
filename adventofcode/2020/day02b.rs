#![feature(str_split_once)]

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let result = input
        .lines()
        .map(|line| {
            let (part, rem) = line.split_once("-").unwrap();
            let i: usize = part.parse().unwrap();

            let (part, rem) = rem.split_once(" ").unwrap();
            let j: usize = part.parse().unwrap();

            let (part, rem) = rem.split_once(": ").unwrap();
            let c: char = part.parse().unwrap();

            (i, j, c, rem)
        })
        .filter(|&(i, j, c, pass)| {
            (pass.chars().nth(i - 1).unwrap() == c) ^ (pass.chars().nth(j - 1).unwrap() == c)
        })
        .count();

    println!("{}", result);
}
