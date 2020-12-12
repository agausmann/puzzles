use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut adapters: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let ones = adapters.windows(2).filter(|v| v[1] - v[0] == 1).count();
    let threes = adapters.windows(2).filter(|v| v[1] - v[0] == 3).count();

    println!("{}", ones * threes);
}
