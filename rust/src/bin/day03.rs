use rust::utils::read_input;

#[allow(dead_code)]
const TEST_STRING: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn find_max_pair_comb_in_line(line: &[u8]) -> i32 {
    let mut max = 0;

    for i in 0..(line.len() - 1) {
        for j in (i + 1)..line.len() {
            max = max.max(line[i] as i32 * 10 + line[j] as i32);
        }
    }
    max
}

fn find_max_12_comb_in_line(line: &[u8]) -> i64 {
    let mut dp = vec![0; 12];

    for &num in line {
        for size in (1..12).rev() {
            dp[size] = dp[size].max(dp[size - 1] * 10 + num as i64);
        }
        dp[0] = dp[0].max(num as i64);
    }
    dp[11]
}

fn main() {
    let input = read_input(3).expect("Failed to read input for day 3");
    let now = std::time::Instant::now();
    let grid = parse_grid(&input);

    let (part1, part2) = grid
        .iter()
        .map(|line| {
            let p1 = find_max_pair_comb_in_line(line);
            let p2 = find_max_12_comb_in_line(line);
            (p1, p2)
        })
        .fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    println!("Part 1: Total output joltage is {}", part1);
    println!("Part 2: Total output joltage is {}", part2);
    println!("Elapsed time: {:?}", now.elapsed());
}
