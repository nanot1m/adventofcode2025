use rust::utils::read_input_lines;

const START: i32 = 50;

fn main() {
    let input = read_input_lines(1).expect("Failed to read input for day 1");

    let lines = input
        .iter()
        .map(|l| {
            let (head, tail) = l.split_at(1);
            let dist: i32 = tail.parse().unwrap();
            match head {
                "L" => -dist,
                "R" => dist,
                _ => panic!("invalid direction"),
            }
        })
        .collect::<Vec<i32>>();

    let part1 = lines
        .iter()
        .scan(START, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .filter(|x| *x % 100 == 0)
        .count();

    println!("Part 1: Zero count is {}", part1);

    let part2 = lines
        .iter()
        .flat_map(|x| std::iter::repeat(x.signum()).take(x.abs() as usize))
        .scan(START, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .filter(|x| *x % 100 == 0)
        .count();

    println!("Part 2: Zero count is {}", part2);
}
