use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let result = input
        .lines()
        .map(|mut line| parse_expr(&mut line))
        .sum::<u64>();

    println!("{}", result);
}

fn parse_expr(mut rem: &mut &str) -> u64 {
    *rem = rem.trim_start();
    let mut acc = if rem.starts_with('(') {
        *rem = &rem[1..];
        parse_expr(&mut rem)
    } else {
        parse_int(&mut rem)
    };

    loop {
        *rem = rem.trim_start();

        if rem.is_empty() {
            break;
        } else if rem.starts_with(')') {
            *rem = &rem[1..];
            break;
        } else if rem.starts_with('+') {
            *rem = rem[1..].trim_start();
            let rhs = if rem.starts_with('(') {
                *rem = &rem[1..];
                parse_expr(&mut rem)
            } else {
                parse_int(&mut rem)
            };
            acc += rhs;
        } else if rem.starts_with('*') {
            *rem = rem[1..].trim_start();
            let rhs = if rem.starts_with('(') {
                *rem = &rem[1..];
                parse_expr(&mut rem)
            } else {
                parse_int(&mut rem)
            };
            acc *= rhs;
        } else {
            panic!("near {:?}", rem);
        }
    }

    acc
}

fn parse_int(rem: &mut &str) -> u64 {
    let end = rem
        .char_indices()
        .skip_while(|&(_, c)| c.is_ascii_digit())
        .map(|(i, _)| i)
        .next()
        .unwrap_or(rem.len());

    let int = rem[..end].parse().unwrap();
    *rem = &rem[end..];
    int
}
