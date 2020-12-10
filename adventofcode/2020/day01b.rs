use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let handle = stdin.lock();

    let entries: Vec<usize> = handle
        .lines()
        .map(Result::unwrap)
        .filter_map(|line| line.parse::<usize>().ok())
        .filter(|&x| x <= 2020)
        .collect();

    let mut seen = [false; 2021];
    for &x in &entries {
        seen[x] = true;
    }

    'outer: for &x in &entries {
        for &y in &entries {
            let z = x + y;
            if z <= 2020 && seen[2020 - z] {
                println!("{}", x * y * (2020 - z));
                break 'outer;
            }
        }
    }
}
