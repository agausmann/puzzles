use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let data: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    for i in 25..data.len() {
        let valid = (0..25)
            .flat_map(|j| ((j + 1)..26).map(move |k| (j, k)))
            .any(|(j, k)| data[i - j] + data[i - k] == data[i]);
        if !valid {
            println!("{}", data[i]);
            break;
        }
    }
}
