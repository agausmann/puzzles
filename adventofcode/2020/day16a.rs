#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut sections = input.split("\n\n");

    let mut field_rules: HashMap<&str, Vec<(u32, u32)>> = HashMap::new();
    for line in sections.next().unwrap().lines() {
        let (key, rule_str) = line.split_once(": ").unwrap();
        let rules = rule_str
            .split(" or ")
            .map(|single_rule| {
                let (lower, upper) = single_rule.split_once("-").unwrap();
                (lower.parse::<u32>().unwrap(), upper.parse::<u32>().unwrap())
            })
            .collect();
        field_rules.insert(key, rules);
    }

    let _ = sections.next();

    let invalids = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|line| line.split(",").map(|s| s.parse::<u32>().unwrap()))
        .filter(|&x| {
            !field_rules
                .values()
                .any(|rule| rule.iter().any(|&(lower, upper)| x >= lower && x <= upper))
        })
        .sum::<u32>();

    println!("{}", invalids);
}
