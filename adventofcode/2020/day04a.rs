#![feature(str_split_once)]

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let passports = input.split("\n\n").map(|parts| {
        parts
            .split_whitespace()
            .map(|part| part.split_once(":").unwrap())
    });

    let valids = passports
        .filter(|parts| {
            let mut byr = false;
            let mut iyr = false;
            let mut eyr = false;
            let mut hgt = false;
            let mut hcl = false;
            let mut ecl = false;
            let mut pid = false;
            for (name, _value) in parts.clone() {
                match name {
                    "byr" => byr = true,
                    "iyr" => iyr = true,
                    "eyr" => eyr = true,
                    "hgt" => hgt = true,
                    "hcl" => hcl = true,
                    "ecl" => ecl = true,
                    "pid" => pid = true,
                    _ => {}
                }
            }
            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count();

    println!("{}", valids);
}
