use std::io::{stdin, Read};
use std::ops;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let result = input
        .lines()
        .map(|mut line| parse_expr(&mut line))
        .sum::<u64>();

    println!("{}", result);
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Int(u64),
    Add,
    Mul,
}

impl ops::AddAssign for Token {
    fn add_assign(&mut self, rhs: Token) {
        match (self, rhs) {
            (Token::Int(x), Token::Int(y)) => {
                *x += y;
            }
            _ => unreachable!(),
        }
    }
}

impl ops::MulAssign for Token {
    fn mul_assign(&mut self, rhs: Token) {
        match (self, rhs) {
            (Token::Int(x), Token::Int(y)) => {
                *x *= y;
            }
            _ => unreachable!(),
        }
    }
}

fn parse_expr(mut rem: &mut &str) -> u64 {
    *rem = rem.trim_start();
    let mut tokens = Vec::new();

    if rem.starts_with('(') {
        *rem = &rem[1..];
        tokens.push(Token::Int(parse_expr(&mut rem)));
    } else {
        tokens.push(Token::Int(parse_int(&mut rem)));
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
            tokens.push(Token::Add);
        } else if rem.starts_with('*') {
            *rem = rem[1..].trim_start();
            tokens.push(Token::Mul);
        } else {
            panic!("near {:?}", rem);
        }

        if rem.starts_with('(') {
            *rem = &rem[1..];
            tokens.push(Token::Int(parse_expr(&mut rem)));
        } else {
            tokens.push(Token::Int(parse_int(&mut rem)));
        }
    }

    let mut i = 0;
    let mut j = 0;

    while j < tokens.len() {
        match tokens[j] {
            Token::Int(..) | Token::Mul => {
                tokens[i] = tokens[j];
                i += 1;
                j += 1;
            }
            Token::Add => {
                let x = tokens[j + 1];
                tokens[i - 1] += x;
                j += 2;
            }
        }
    }
    tokens.truncate(i);

    let i = 1;
    let mut j = 1;

    while j < tokens.len() {
        match tokens[j] {
            Token::Int(..) | Token::Add => unreachable!(),
            Token::Mul => {
                let x = tokens[j + 1];
                tokens[i - 1] *= x;
                j += 2;
            }
        }
    }
    tokens.truncate(i);

    assert!(tokens.len() == 1);
    match tokens[0] {
        Token::Int(x) => x,
        _ => unreachable!(),
    }
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
