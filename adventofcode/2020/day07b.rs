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

    fn bag_count(adj: &HashMap<&str, HashMap<&str, usize>>, key: &str) -> usize {
        adj[key]
            .iter()
            .map(|(inner_key, count)| count * (bag_count(adj, inner_key) + 1))
            .sum()
    }

    println!("{}", bag_count(&adj, "shiny gold"));
}
