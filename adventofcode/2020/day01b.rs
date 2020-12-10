use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let entries: Vec<usize> = input
        .lines()
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
