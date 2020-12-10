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
            let c: u8 = part.as_bytes()[0];

            (i, j, c, rem.as_bytes())
        })
        .filter(|&(i, j, c, pass)| {
            let count = pass.iter().filter(|&&d| d == c).count();
            count >= i && count <= j
        })
        .count();

    println!("{}", result);
}
