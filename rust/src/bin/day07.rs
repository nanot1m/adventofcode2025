use rust::utils::{Point, get_grid_point, iterate_grid, parse_char_grid, read_input};
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
const TEST_STRING: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

fn recursive_count(pos: Point, grid: &[Vec<char>], cache: &mut HashMap<Point, i64>) -> i64 {
    if let Some(&count) = cache.get(&pos) {
        return count;
    }

    let cell = get_grid_point(grid, pos);
    let count = match cell {
        Some('^') => {
            let left_count  = recursive_count(Point { x: pos.x - 1, y: pos.y }, grid, cache);
            let right_count = recursive_count(Point { x: pos.x + 1, y: pos.y }, grid, cache);
            left_count + right_count
        }
        Some(_) => recursive_count(Point { x: pos.x, y: pos.y + 1 }, grid, cache),
        None => 1,
    };

    cache.insert(pos, count);
    count
}

fn main() {
    let input = read_input(7).expect("Failed to read input for day 7");
    let now = std::time::Instant::now();

    let grid = parse_char_grid(&input);

    let start = iterate_grid(&grid)
        .find(|&(_, cell)| cell == 'S')
        .map(|(pos, _)| pos)
        .expect("No starting position 'S' found");

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::from([start]);
    let mut part1 = 0;

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let next_pos = Point { x: pos.x, y: pos.y + 1 };
        let next_cell = get_grid_point(&grid, next_pos);
        match next_cell {
            Some('^') => {
                part1 += 1;
                queue.push_back(Point { x: next_pos.x - 1, y: next_pos.y });
                queue.push_back(Point { x: next_pos.x + 1, y: next_pos.y });
            }
            Some(_) => queue.push_back(next_pos),
            None => {}
        }
    }

    println!("Part 1: {}", part1);

    let part2 = recursive_count(start, &grid, &mut HashMap::new());
    println!("Part 2: {}", part2);
    println!("Elapsed time: {:?}", now.elapsed());
}
