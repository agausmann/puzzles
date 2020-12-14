#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut mem = HashMap::new();
    let mut or_mask = 0;
    let mut floating = Vec::new();

    fn float<F>(addr: u64, floating: &[usize], func: &mut F)
    where
        F: FnMut(u64),
    {
        if floating.len() == 0 {
            func(addr);
        } else {
            float(addr & !(1 << floating[0]), &floating[1..], func);
            float(addr | (1 << floating[0]), &floating[1..], func);
        }
    }

    for line in input.lines() {
        if let Some(rem) = line.strip_prefix("mask = ") {
            or_mask = 0;
            floating.clear();
            for (i, c) in rem.chars().enumerate() {
                or_mask <<= 1;
                match c {
                    'X' => {
                        floating.push(35 - i);
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

            float(addr | or_mask, &floating, &mut |modified_addr| {
                mem.insert(modified_addr, value);
            });
        }
    }

    println!("{}", mem.values().copied().sum::<u64>());
}
