#![feature(str_split_once)]

use std::io::{stdin, Read};
use std::mem;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut code: Vec<(&str, i64)> = input
        .lines()
        .map(|line| {
            let (op, arg) = line.split_once(" ").unwrap();
            let arg: i64 = arg.parse().unwrap();
            (op, arg)
        })
        .collect();

    for i in 0..code.len() {
        // if this is a nop/jmp, swap and test if it halts.
        let new_op = match code[i].0 {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => continue,
        };
        let old_op = mem::replace(&mut code[i].0, new_op);

        let mut seen: Vec<bool> = vec![false; code.len()];
        let mut pc: i64 = 0;
        let mut acc: i64 = 0;

        loop {
            if pc as usize >= code.len() {
                println!("{}", acc);
                return;
            }
            if seen[pc as usize] {
                break;
            }
            seen[pc as usize] = true;
            let (op, arg) = code[pc as usize];
            match op {
                "acc" => {
                    acc += arg;
                    pc += 1;
                }
                "jmp" => {
                    pc += arg;
                }
                "nop" => {
                    pc += 1;
                }
                _ => panic!("unknown op {}", op),
            }
        }

        // restore old operation
        code[i].0 = old_op;
    }
}
