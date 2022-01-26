use std::str::Lines;
fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> Lines<'static> {
  raw_input().lines()
}

fn solve(line: &str) -> i32 {
  let mut count = 0;
  let parts: Vec<&str> = line.split('|').collect();
  assert_eq!(parts.len(), 2);
  let digits = parts[1].split(' ');
  for d in digits {
    match d.len() {
      2 | 3 | 4 | 7 => count += 1,
      _ => (),
    }
  }
  count
}

fn main() {
  let mut total = 0;
  for l in input() {
    total += solve(l);
  }
  println!("{}", total)
}
