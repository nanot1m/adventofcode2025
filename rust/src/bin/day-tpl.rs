use rust::utils::read_input;

#[allow(dead_code)]
const TEST_STRING: &str = "\
";

const DAY: usize = 1;

fn main() {
    let input = read_input(DAY).expect(format!("Failed to read input for day {}", DAY).as_str());
    let now = std::time::Instant::now();

    let part1 = 0;
    println!("Part 1: {}", part1);
    let part2 = 0;
    println!("Part 2: {}", part2);
    println!("Elapsed time: {:?}", now.elapsed());
}
