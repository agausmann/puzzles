#![feature(str_split_once)]

use std::collections::{BTreeMap, BTreeSet};
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut sections = input.split("\n\n");

    let mut field_rules: BTreeMap<&str, Vec<(u64, u64)>> = BTreeMap::new();
    for line in sections.next().unwrap().lines() {
        let (key, rule_str) = line.split_once(": ").unwrap();
        let rules = rule_str
            .split(" or ")
            .map(|single_rule| {
                let (lower, upper) = single_rule.split_once("-").unwrap();
                (lower.parse::<u64>().unwrap(), upper.parse::<u64>().unwrap())
            })
            .collect();
        field_rules.insert(key, rules);
    }

    let mine: Vec<u64> = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let nearby: Vec<Vec<u64>> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|ticket| {
            ticket.iter().all(|&x| {
                field_rules
                    .values()
                    .any(|rule| rule.iter().any(|&(lower, upper)| x >= lower && x <= upper))
            })
        })
        .collect();

    let mut possibilities: Vec<BTreeSet<&str>> =
        vec![field_rules.keys().copied().collect(); nearby[0].len()];

    for ticket in nearby {
        for (&x, set) in ticket.iter().zip(&mut possibilities) {
            let valids: BTreeSet<&str> = field_rules
                .iter()
                .filter(|(_, rules)| rules.iter().any(|&(lower, upper)| x >= lower && x <= upper))
                .map(|(&key, _)| key)
                .collect();
            *set = set.intersection(&valids).copied().collect();
        }
    }

    let mut changes = true;
    while changes {
        changes = false;

        let solved: BTreeSet<&str> = possibilities
            .iter()
            .filter(|set| set.len() == 1)
            .flatten()
            .copied()
            .collect();

        for set in &mut possibilities {
            if set.len() > 1 {
                let len = set.len();
                *set = set.difference(&solved).copied().collect();
                if len != set.len() {
                    changes = true;
                }
            }
        }
    }

    let result: u64 = possibilities
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, s)| s.starts_with("departure"))
        .map(|(i, _)| mine[i])
        .product();

    println!("{}", result);
}
