use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut adapters: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    fn combinations(adapters: &[u64], cache: &mut HashMap<u64, usize>) -> usize {
        if adapters.len() <= 1 {
            return 1;
        }
        let this = adapters[0];
        if let Some(&cached) = cache.get(&this) {
            return cached;
        }
        let result = adapters
            .iter()
            .copied()
            .enumerate()
            .skip(1)
            .take_while(|&(_, x)| x - this <= 3)
            .map(|(i, _)| combinations(&adapters[i..], cache))
            .sum();
        cache.insert(this, result);
        result
    }

    println!("{}", combinations(&adapters, &mut HashMap::new()));
}
