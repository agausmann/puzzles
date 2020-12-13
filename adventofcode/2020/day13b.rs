use std::io::{stdin, Read};

fn main() {
    //tests
    assert_eq!(bezout(3, 4), (-1, 1));
    assert_eq!(bezout(5, 12), (5, -2));

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let _arrival: i128 = lines.next().unwrap().parse().unwrap();
    let mut buses = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(offset, id)| {
            let id = id.parse::<i128>().unwrap();
            (id - (offset as i128), id)
        });

    let (mut a1, mut n1) = buses.next().unwrap();
    for (a2, n2) in buses {
        let (m1, m2) = bezout(n1, n2);
        a1 = ((a1 * m2 * n2) + (a2 * m1 * n1)) % (n1 * n2);
        n1 *= n2;
    }
    println!("{}", (a1 % n1 + n1) % n1);
}

fn bezout(a: i128, b: i128) -> (i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;

        let tmp = r;
        r = old_r - q * tmp;
        old_r = tmp;

        let tmp = s;
        s = old_s - q * tmp;
        old_s = tmp;

        let tmp = t;
        t = old_t - q * tmp;
        old_t = tmp;
    }
    (old_s, old_t)
}
