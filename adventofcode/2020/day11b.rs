#![feature(str_split_once)]

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    loop {
        let mut grid2 = grid.clone();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                //dbg!((i, j));
                let mut neighbors = 0;
                let mut count = |a: isize, b: isize| {
                    let mut k = 1;
                    loop {
                        let c = grid
                            .get(((i as isize) + (k * a)) as usize)
                            .and_then(|row| row.get(((j as isize) + (k * b)) as usize));
                        //dbg!((k, c));
                        match grid
                            .get(((i as isize) + (k * a)) as usize)
                            .and_then(|row| row.get(((j as isize) + (k * b)) as usize))
                        {
                            Some('L') => break,
                            Some('#') => {
                                neighbors += 1;
                                break;
                            }
                            None => break,
                            _ => {}
                        }
                        k += 1;
                    }
                };
                count(0, 1);
                count(0, -1);
                count(1, 0);
                count(-1, 0);
                count(1, 1);
                count(-1, 1);
                count(-1, -1);
                count(1, -1);
                grid2[i][j] = match (grid[i][j], neighbors) {
                    ('L', 0) => '#',
                    ('#', 5..=10000) => 'L',
                    (x, _) => x,
                };
            }
        }
        /*
        for row in &grid2 {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        break;
        */
        if grid2 == grid {
            break;
        }
        grid = grid2;
    }

    println!(
        "{}",
        grid.into_iter().flatten().filter(|&c| c == '#').count()
    );
}
