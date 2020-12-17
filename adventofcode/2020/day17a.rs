use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut state: HashSet<(i32, i32, i32)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                state.insert((x as i32, y as i32, 0));
            }
        }
    }

    for _i in 0..6 {
        let mut neighbors = HashMap::new();
        for &(x, y, z) in &state {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if (dx, dy, dz) == (0, 0, 0) {
                            continue;
                        }

                        *neighbors.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
                    }
                }
            }
        }

        state.retain(|key| matches!(neighbors.get(key).copied().unwrap_or(0), 2 | 3));
        for (key, count) in neighbors {
            if count == 3 {
                state.insert(key);
            }
        }
    }

    println!("{}", state.len());
}
