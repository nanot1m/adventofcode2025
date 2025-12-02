use rust::utils::read_input;

#[allow(dead_code)]
const TEST_STRING: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn parse_range(s: &str) -> (i64, i64) {
    let parts: Vec<&str> = s.split('-').collect();
    let start: i64 = parts[0].parse().unwrap();
    let end: i64 = parts[1].parse().unwrap();
    (start, end)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input.split(',').map(|r| parse_range(r)).collect()
}

fn is_invalid_id(id: &i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    let (left, right) = id_str.split_at(len.div_ceil(2));
    left == right
}

fn is_invalid_id_v2(id: &i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    for size in 1..=(len / 2) {
        if len % size != 0 {
            continue;
        }
        let pattern = &id_str[0..size];
        let mut repeated = String::new();
        for _ in 0..(len / size) {
            repeated.push_str(pattern);
        }
        if repeated == id_str {
            return true;
        }
    }
    false
}

fn main() {
    let input = read_input(2)
        .expect("Failed to read input for day 2")
        .trim_end()
        .to_string();
    let ranges = parse_input(&input);

    let part1: i64 = ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(is_invalid_id)
        .sum();

    println!("Part 1: Total invalid IDs sum: {}", part1);

    let part2: i64 = ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(is_invalid_id_v2)
        .sum();

    println!("Part 2: Total invalid IDs sum: {}", part2);
}
