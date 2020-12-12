use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let data: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut invalid = None;

    for i in 25..data.len() {
        let valid = (0..25)
            .flat_map(|j| ((j + 1)..26).map(move |k| (j, k)))
            .any(|(j, k)| data[i - j] + data[i - k] == data[i]);
        if !valid {
            invalid = Some(data[i]);
            break;
        }
    }

    let invalid = invalid.unwrap();

    let mut head = 0;
    let mut tail = 0;
    let mut acc = 0;
    while acc != invalid {
        if acc < invalid || (head - tail) <= 2 {
            acc += data[tail];
            tail += 1;
        } else if acc > invalid {
            acc -= data[head];
            head += 1;
        }
    }

    let weakness = data[head..tail].iter().copied().max().unwrap()
        + data[head..tail].iter().copied().min().unwrap();

    println!("{}", weakness);
}
