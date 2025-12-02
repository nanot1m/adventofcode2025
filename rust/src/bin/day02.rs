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

fn is_invalid_id_str(id_str: &str) -> bool {
    let len = id_str.len();
    let (left, right) = id_str.split_at(len.div_ceil(2)); // ceil(len/2)
    left == right
}

fn is_invalid_id_repeating(id_str: &str) -> bool {
    let bytes = id_str.as_bytes();
    let len = bytes.len();

    (1..=(len / 2))
        .filter(|size| len % size == 0)
        .any(|size| {
            let first = &bytes[..size];
            bytes.chunks(size).all(|chunk| chunk == first)
        })
}

fn main() {
    let input = read_input(2)
        .expect("Failed to read input for day 2")
        .trim_end()
        .to_string();
    let ranges = parse_input(&input);

    let (part1, part2) = ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .fold((0i64, 0i64), |(p1, p2), id| {
            let id_str = id.to_string();
            let p1_next = if is_invalid_id_str(&id_str) { p1 + id } else { p1 };
            let p2_next = if is_invalid_id_repeating(&id_str) { p2 + id } else { p2 };
            (p1_next, p2_next)
        });

    println!("Part 1: Total invalid IDs sum: {}", part1);
    println!("Part 2: Total invalid IDs sum: {}", part2);
}
