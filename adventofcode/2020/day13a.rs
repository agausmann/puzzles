use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let arrival: u64 = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&s| s != "x")
        .map(|s| s.parse::<u64>().unwrap());

    let (bus_id, wait_time) = buses
        .map(|bus| (bus, bus - (arrival % bus)))
        .min_by_key(|&(_, wait)| wait)
        .unwrap();

    println!("{}", bus_id * wait_time);
}
