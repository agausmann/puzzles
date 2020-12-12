#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let adj: HashMap<&str, HashMap<&str, usize>> = input
        .lines()
        .map(|line| {
            let (node, rem) = line.split_once(" bags contain ").unwrap();
            let adj = if rem == "no other bags." {
                HashMap::new()
            } else {
                rem.split(", ")
                    .map(|part| {
                        let (count, rem) = part.split_once(" ").unwrap();
                        let count: usize = count.parse().unwrap();

                        let (color, _) = rem.split_once(" bag").unwrap();
                        (color, count)
                    })
                    .collect()
            };
            (node, adj)
        })
        .collect();

    fn contains_shiny_gold(adj: &HashMap<&str, HashMap<&str, usize>>, key: &str) -> bool {
        adj[key]
            .keys()
            .any(|&inner_key| inner_key == "shiny gold" || contains_shiny_gold(adj, inner_key))
    }

    let count = adj
        .keys()
        .filter(|&&key| contains_shiny_gold(&adj, key))
        .count();
    println!("{}", count);
}
