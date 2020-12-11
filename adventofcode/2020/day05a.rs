use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let ids = input.lines().map(|line| {
        line.chars().fold(0, |acc, c| match c {
            'F' | 'L' => acc << 1,
            'B' | 'R' => (acc << 1) + 1,
            _ => panic!("unexpected char {}", c),
        })
    });

    println!("{}", ids.max().unwrap());
}
