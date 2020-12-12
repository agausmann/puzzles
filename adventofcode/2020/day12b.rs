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
    let mut wx = 10;
    let mut wy = 1;

    for (direction, amount) in directions {
        match (direction, amount) {
            ("N", _) => wy += amount,
            ("S", _) => wy -= amount,
            ("E", _) => wx += amount,
            ("W", _) => wx -= amount,
            ("F", _) => {
                x += wx * amount;
                y += wy * amount;
            }
            ("L", 0) | ("R", 0) => {}
            ("L", 90) | ("R", 270) => {
                swap(&mut wx, &mut wy);
                wx = -wx;
            }
            ("L", 180) | ("R", 180) => {
                wx = -wx;
                wy = -wy;
            }
            ("L", 270) | ("R", 90) => {
                swap(&mut wx, &mut wy);
                wy = -wy;
            }
            _ => panic!(),
        }
    }

    println!("{}", x.abs() + y.abs());
}
