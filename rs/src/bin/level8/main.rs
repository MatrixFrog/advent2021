use std::collections::HashSet;
use std::str::Lines;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> Lines<'static> {
  raw_input().lines()
}

fn parse_half_line(half_line: &str) -> Vec<HashSet<char>> {
  half_line
    .trim()
    .split(' ')
    .map(|digit| digit.chars().collect::<HashSet<char>>())
    .collect()
}

fn parse_line(line: &str) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
  let parts: Vec<&str> = line.split('|').collect();
  assert_eq!(parts.len(), 2);
  let all_ten = parse_half_line(parts[0]);
  let four = parse_half_line(parts[1]);
  assert_eq!(all_ten.len(), 10);
  assert_eq!(four.len(), 4);
  (all_ten, four)
}

// Get the only digit with the given number of segments, from the list of all ten digits
fn get_only(all_ten: &[HashSet<char>], count: usize) -> &HashSet<char> {
  let ones = all_ten
    .iter()
    .filter(|d| d.len() == count)
    .collect::<Vec<&HashSet<char>>>();
  assert_eq!(ones.len(), 1);
  ones[0]
}

fn get_one(all_ten: &[HashSet<char>]) -> &HashSet<char> {
  get_only(all_ten, 2)
}

fn get_four(all_ten: &[HashSet<char>]) -> &HashSet<char> {
  get_only(all_ten, 4)
}

// 0 -> 6
// 1 -> 2 *
// 2 -> 5
// 3 -> 5
// 4 -> 4 *
// 5 -> 5
// 6 -> 6
// 7 -> 3 *
// 8 -> 7 *
// 9 -> 6
fn solve_digit(all_ten: &[HashSet<char>], digit: HashSet<char>) -> i32 {
  match digit.len() {
    2 => 1,
    4 => 4,
    3 => 7,
    7 => 8,
    5 => {
      // Could be 2, 3, or 5

      // Of these, 3 is the only one that has both the segments from '1' lit up.
      if digit.is_superset(get_one(all_ten)) {
        return 3;
      }

      // To distinguish between 2 and 5, look at the 4:
      // 2 has 2 segments in common with 4; 5 has 3 segments in common with 4.
      let four = get_four(all_ten);
      match digit.intersection(four).count() {
        2 => 2,
        3 => 5,
        l => {
          panic!("Unexpected segment overlap with 4: {}", l)
        }
      }
    }
    6 => {
      // Could be 0, 6, or 9

      // Of these, 6 is the only one that doesn't have both the segments from '1' lit up.
      if !digit.is_superset(get_one(all_ten)) {
        return 6;
      }

      // 9 has all 4 segments from '4' lit up.
      if digit.is_superset(get_four(all_ten)) {
        return 9;
      }

      0
    }
    l => {
      panic!("Unexpected length, {}", l);
    }
  }
}

fn solve(line: &str) -> i32 {
  let mut total = 0;
  let (all_ten, four) = parse_line(line);
  for digit in four {
    total = 10 * total + solve_digit(&all_ten, digit);
  }
  total
}

fn main() {
  let mut total = 0;
  for l in input() {
    total += solve(l)
  }
  println!("{}", total)
}
