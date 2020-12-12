#![feature(str_split_once)]

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let code: Vec<(&str, i64)> = input
        .lines()
        .map(|line| {
            let (op, arg) = line.split_once(" ").unwrap();
            let arg: i64 = arg.parse().unwrap();
            (op, arg)
        })
        .collect();

    let mut seen: Vec<bool> = vec![false; code.len()];
    let mut pc: i64 = 0;
    let mut acc: i64 = 0;

    loop {
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

    println!("{}", acc);
}
