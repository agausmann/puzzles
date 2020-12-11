use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let sum: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.as_bytes())
                .fold(0u32, |acc, b| acc | (1 << (b - b'a')))
                .count_ones()
        })
        .sum();

    println!("{}", sum);
}
