use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut t = 1;
    let mut last_spoken: HashMap<u64, u64> = HashMap::new();
    let mut next: u64 = 0;

    for num in input.trim().split(",") {
        last_spoken.insert(num.parse::<u64>().unwrap(), t);
        t += 1;
    }

    while t < 2020 {
        let last = last_spoken.insert(next, t);
        if let Some(last) = last {
            next = t - last;
        } else {
            next = 0;
        }
        t += 1;
    }
    println!("{}", next);
}
