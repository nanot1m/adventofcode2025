use rust::utils::read_input;

#[allow(dead_code)]
const TEST_STRING: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

fn transpose_grid<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let first_elem = grid[0][0];
    let mut transposed = vec![vec![first_elem; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = grid[i][j];
        }
    }

    transposed
}

fn transpose_char_grid(grid: &[&str]) -> Vec<String> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let char_grid: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();
    let cols = char_grid[0].len();
    let mut transposed = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = char_grid[i][j];
        }
    }

    transposed
        .into_iter()
        .map(|col| col.into_iter().collect())
        .collect()
}

fn apply_operations(nums_grid: &[Vec<i64>], ops: &[char]) -> i64 {
    nums_grid
        .iter()
        .zip(ops.iter())
        .map(|(nums, &op)| match op {
            '*' => nums.iter().product::<i64>(),
            '+' => nums.iter().sum::<i64>(),
            _ => panic!("Invalid operation: {}", op),
        })
        .sum()
}

fn parse_number_grid(lines: &[String]) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect()
        })
        .collect()
}

fn main() {
    let input = read_input(6).expect("Failed to read input for day 6");
    let lines: Vec<&str> = input.lines().collect();

    let (nums_lines, ops_lines) = lines.split_at(lines.len() - 1);

    let ops: Vec<char> = ops_lines[0]
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    let nums_grid_rows: Vec<Vec<i64>> = nums_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect()
        })
        .collect();

    let nums_grid_cols = transpose_grid(&nums_grid_rows);
    let part1 = apply_operations(&nums_grid_cols, &ops);
    println!("Part 1: {}", part1);

    let transposed_strings = transpose_char_grid(nums_lines);

    let grouped_strings: Vec<String> = transposed_strings
        .split(|s| s.trim().is_empty())
        .filter(|group| !group.is_empty())
        .map(|group| group.join(" "))
        .collect();

    let nums_grid_part2 = parse_number_grid(&grouped_strings);
    let part2 = apply_operations(&nums_grid_part2, &ops);
    println!("Part 2: {}", part2);
}
