use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let sum: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.as_bytes()
                        .iter()
                        .fold(0u32, |acc, b| acc | (1 << (b - b'a')))
                })
                .fold(0x3ffffffu32, |acc, set| acc & set)
                .count_ones()
        })
        .sum();

    println!("{}", sum);
}
