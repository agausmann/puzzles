use std::io::{stdin, Read};

const ROWS: usize = 128;
const COLS: usize = 8;

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

    let mut occupied = [false; ROWS * COLS];

    for id in ids {
        occupied[id] = true;
    }

    for (i, v) in occupied.windows(3).enumerate() {
        if v == [true, false, true] {
            println!("{}", i + 1);
            break;
        }
    }
}
