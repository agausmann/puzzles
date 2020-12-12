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
                let mut neighbors = 0;
                if i > 0 && j > 0 && grid[i - 1][j - 1] == '#' {
                    neighbors += 1;
                }
                if i > 0 && grid[i - 1][j] == '#' {
                    neighbors += 1;
                }
                if j > 0 && grid[i][j - 1] == '#' {
                    neighbors += 1;
                }
                if i < grid.len() - 1 && grid[i + 1][j] == '#' {
                    neighbors += 1;
                }
                if j < grid[0].len() - 1 && grid[i][j + 1] == '#' {
                    neighbors += 1;
                }
                if i > 0 && j < grid[0].len() - 1 && grid[i - 1][j + 1] == '#' {
                    neighbors += 1;
                }
                if i < grid.len() - 1 && j > 0 && grid[i + 1][j - 1] == '#' {
                    neighbors += 1;
                }
                if i < grid.len() - 1 && j < grid[0].len() - 1 && grid[i + 1][j + 1] == '#' {
                    neighbors += 1;
                }
                grid2[i][j] = match (grid[i][j], neighbors) {
                    ('L', 0) => '#',
                    ('#', 4..=10000) => 'L',
                    (x, _) => x,
                };
            }
        }
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
