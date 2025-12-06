use rust::utils::read_input;

#[allow(dead_code)]
const TEST_STRING: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn merge_ranges(ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_by_key(|r| r.0);
    let mut merged = vec![ranges[0]];

    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

fn point_in_range(point: i64, ranges: &[(i64, i64)]) -> bool {
    for &(start, end) in ranges {
        if point >= start && point <= end {
            return true;
        }
    }
    false
}

fn main() {
    let input = read_input(5).expect("Failed to read input for day 5");

    let mut parts = input.split("\n\n");
    let ranges = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('-').map(|n| n.parse::<i64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let points = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let merged_ranges = merge_ranges(&mut ranges.clone());

    let part1 = points
        .iter()
        .filter(|&&p| point_in_range(p, &merged_ranges))
        .count();

    println!("Part 1: {}", part1);

    let part2 = merged_ranges.iter().map(|(s, e)| e - s + 1).sum::<i64>();

    println!("Part 2: {}", part2);
}
