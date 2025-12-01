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