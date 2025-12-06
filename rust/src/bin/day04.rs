use rust::utils::{read_input, iterate_neighbors_8, parse_char_grid};
use std::collections::HashSet;

#[allow(dead_code)]
const TEST_STRING: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn can_access_roll_of_paper(pos: (usize, usize), grid: &[Vec<char>]) -> bool {
    let count = iterate_neighbors_8(pos, grid)
        .iter()
        .filter(|(value, _)| **value == '@')
        .count();
    count < 4
}

fn main() {
    let input = read_input(4).expect("Failed to read input for day 4");
    let now = std::time::Instant::now();

    let grid = parse_char_grid(&input);

    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' && can_access_roll_of_paper((i, j), &grid) {
                count += 1;
            }
        }
    }

    println!("Part 1: Total rolls can be accessed {}", count);

    let mut count_part2 = 0;
    let mut cur_grid = grid;

    loop {
        let mut to_remove = HashSet::new();

        for (i, row) in cur_grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '@' && can_access_roll_of_paper((i, j), &cur_grid) {
                    to_remove.insert((i, j));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for &(i, j) in &to_remove {
            cur_grid[i][j] = '.';
        }

        count_part2 += to_remove.len();
    }

    println!("Part 2: Total rolls can be removed {}", count_part2);
    println!("Elapsed time: {:?}", now.elapsed());
}
