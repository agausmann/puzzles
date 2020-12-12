use std::io::{stdin, Read};
use std::mem::swap;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let directions = input.lines().map(|line| {
        let direction = &line[..1];
        let amount: i64 = line[1..].parse().unwrap();
        (direction, amount)
    });

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut facing_x = 1;
    let mut facing_y = 0;

    for (direction, amount) in directions {
        match (direction, amount) {
            ("N", _) => y += amount,
            ("S", _) => y -= amount,
            ("E", _) => x += amount,
            ("W", _) => x -= amount,
            ("F", _) => {
                x += facing_x * amount;
                y += facing_y * amount;
            }
            ("L", 0) | ("R", 0) => {}
            ("L", 90) | ("R", 270) => {
                swap(&mut facing_x, &mut facing_y);
                facing_x = -facing_x;
            }
            ("L", 180) | ("R", 180) => {
                facing_x = -facing_x;
                facing_y = -facing_y;
            }
            ("L", 270) | ("R", 90) => {
                swap(&mut facing_x, &mut facing_y);
                facing_y = -facing_y;
            }
            _ => panic!(),
        }
    }

    println!("{}", x.abs() + y.abs());
}
