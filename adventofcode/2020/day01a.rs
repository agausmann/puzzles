use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let entries = input
        .lines()
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
