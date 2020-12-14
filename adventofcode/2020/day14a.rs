#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut mem = HashMap::new();
    let mut or_mask = 0;
    let mut and_mask = !0;

    for line in input.lines() {
        if let Some(rem) = line.strip_prefix("mask = ") {
            or_mask = 0;
            and_mask = !0;
            for c in rem.chars() {
                or_mask <<= 1;
                and_mask <<= 1;
                match c {
                    'X' => {
                        and_mask |= 1;
                    }
                    '0' => {}
                    '1' => {
                        or_mask |= 1;
                    }
                    _ => panic!(),
                }
            }
        } else {
            let rem = line.strip_prefix("mem[").unwrap();
            let (addr, value) = rem.split_once("] = ").unwrap();
            let addr: u64 = addr.parse().unwrap();
            let value: u64 = value.parse().unwrap();
            mem.insert(addr, (value & and_mask) | or_mask);
        }
    }

    println!("{}", mem.values().copied().sum::<u64>());
}
