use std::fs;
use std::io;
use std::path::PathBuf;

/// Read the full input file for the given day (e.g. day=1 -> "../input/day1.input.txt").
/// Returns the file contents as a String (with trailing newline trimmed).
pub fn read_input(day: usize) -> io::Result<String> {
  let path = PathBuf::from(format!("../input/day{}.input.txt", day));
  let mut s = fs::read_to_string(path)?;
  // Trim a single trailing newline (common for puzzle inputs) but preserve other whitespace.
  if s.ends_with('\n') {
    s.pop();
    if s.ends_with('\r') { s.pop(); } // handle CRLF
  }
  Ok(s)
}

/// Convenience to get the input as an iterator of non-owning lines.
pub fn read_input_lines(day: usize) -> io::Result<Vec<String>> {
  let s = read_input(day)?;
  Ok(s.lines().map(|l| l.to_string()).collect())
}

/// Parse input string into a 2D grid of characters.
/// Empty lines are filtered out.
///
/// # Example
/// ```
/// let input = "abc\ndef\nghi";
/// let grid = parse_char_grid(input);
/// assert_eq!(grid[0], vec!['a', 'b', 'c']);
/// ```
pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

/// Parse input string into a 2D grid with a custom mapper function for each character.
/// Empty lines are filtered out.
///
/// # Arguments
/// * `input` - The input string to parse
/// * `mapper` - Function to convert each char to the desired type
///
/// # Example
/// ```
/// let input = "123\n456\n789";
/// let grid = parse_grid_with(input, |c| c.to_digit(10).unwrap() as i32);
/// assert_eq!(grid[0], vec![1, 2, 3]);
/// ```
pub fn parse_grid_with<T, F>(input: &str, mapper: F) -> Vec<Vec<T>>
where
    F: Fn(char) -> T,
{
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(&mapper).collect())
        .collect()
}

/// Iterate over the 8 neighbors (including diagonals) of a position in a 2D grid.
/// Returns a vector of tuples containing (reference to value, position) for each valid neighbor.
///
/// # Arguments
/// * `pos` - The (row, col) position to get neighbors for
/// * `grid` - The 2D grid as a slice of vectors
///
/// # Returns
/// A vector of tuples (&neighbor_value, (neighbor_row, neighbor_col))
///
/// # Example
/// ```
/// let grid = vec![
///     vec!['a', 'b', 'c'],
///     vec!['d', 'e', 'f'],
///     vec!['g', 'h', 'i'],
/// ];
/// let neighbors = iterate_neighbors_8((1, 1), &grid);
/// assert_eq!(neighbors.len(), 8); // 'e' has 8 neighbors
/// ```
pub fn iterate_neighbors_8<T>(pos: (usize, usize), grid: &[Vec<T>]) -> Vec<(&T, (usize, usize))> {
    const NEIGHBORS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let (x, y) = pos;
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut result = Vec::with_capacity(8);

    for &(dx, dy) in &NEIGHBORS {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_y >= 0 && (new_x as usize) < rows && (new_y as usize) < cols {
            let nx = new_x as usize;
            let ny = new_y as usize;
            result.push((&grid[nx][ny], (nx, ny)));
        }
    }

    result
}