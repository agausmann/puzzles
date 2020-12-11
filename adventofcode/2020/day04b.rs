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

            fn num_range(s: &str, lower: u64, upper: u64) -> bool {
                s.parse::<u64>()
                    .map(|x| x >= lower && x <= upper)
                    .unwrap_or(false)
            }

            for (name, value) in parts.clone() {
                match name {
                    "byr" => {
                        if !num_range(value, 1920, 2002) {
                            return false;
                        }
                        byr = true;
                    }
                    "iyr" => {
                        if !num_range(value, 2010, 2020) {
                            return false;
                        }
                        iyr = true;
                    }
                    "eyr" => {
                        if !num_range(value, 2020, 2030) {
                            return false;
                        }
                        eyr = true;
                    }
                    "hgt" => {
                        if let Some(rem) = value.strip_suffix("cm") {
                            if !num_range(rem, 150, 193) {
                                return false;
                            }
                        } else if let Some(rem) = value.strip_suffix("in") {
                            if !num_range(rem, 59, 76) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                        hgt = true;
                    }
                    "hcl" => {
                        if let Some(rem) = value.strip_prefix("#") {
                            if rem.len() != 6 || u32::from_str_radix(rem, 16).is_err() {
                                return false;
                            }
                        } else {
                            return false;
                        }
                        hcl = true;
                    }
                    "ecl" => {
                        match value {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                            _ => {
                                return false;
                            }
                        }
                        ecl = true;
                    }
                    "pid" => {
                        if value.len() != 9 || value.parse::<u64>().is_err() {
                            return false;
                        }
                        pid = true;
                    }
                    _ => {}
                }
            }
            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count();

    println!("{}", valids);
}
