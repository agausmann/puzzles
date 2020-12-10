use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let handle = stdin.lock();

    let entries = handle
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| line.parse::<usize>().ok())
        .filter(|&x| x <= 2020);

    let mut seen = [false; 2021];
    for x in entries {
        if seen[2020 - x] {
            println!("{}", x * (2020 - x));
            break;
        }
        seen[x] = true;
    }
}
