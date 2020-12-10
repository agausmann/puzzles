use std::io::{stdin, Read};

fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();

    let width = input.split(|&c| c == b'\n').next().unwrap().len();

    // Join lines in input
    input.retain(|&c| c != b'\n');

    let trees = |dx: usize, dy: usize| {
        (0..)
            .map(|i| ((dx * i) % width) + dy * i * width)
            .take_while(|&i| i < input.len())
            .filter(|&i| input[i] == b'#')
            .count()
    };

    println!(
        "{}",
        trees(1, 1) * trees(3, 1) * trees(5, 1) * trees(7, 1) * trees(1, 2)
    );
}
