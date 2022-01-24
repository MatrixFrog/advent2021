use std::convert::TryInto;
use std::fmt;

struct Square {
  value: i32,
  marked: bool,
}
impl fmt::Debug for Square {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

#[derive(Debug)]
struct Board {
  data: [[Square; 5]; 5],
  already_won: bool,
}

impl Board {
  fn new(data: &[[i32; 5]; 5]) -> Board {
    Board {
      data: data.map(|row| {
        row.map(|s| Square {
          value: s,
          marked: false,
        })
      }),
      already_won: false,
    }
  }

  // The score of the winning board can now be calculated.
  // Start by finding the sum of all unmarked numbers on that board [...]
  // Then, multiply that sum by the number that was just called when the board won
  fn mark(&mut self, n: i32) -> Option<i32> {
    if self.already_won {
      return None;
    }
    for row in &mut self.data {
      for sq in row {
        if sq.value == n {
          sq.marked = true
        }
      }
    }
    if self.check_for_win() {
      let score = self.sum_of_unmarked() * n;
      self.already_won = true;
      Some(score)
    } else {
      None
    }
  }

  fn check_for_win(&self) -> bool {
    for row in &self.data {
      if row.iter().all(|s| s.marked) {
        return true;
      }
    }
    for i in 0..5 {
      if self.data.iter().all(|r| r[i].marked) {
        return true;
      }
    }
    false
  }

  fn sum_of_unmarked(&self) -> i32 {
    let mut sum = 0;
    for row in &self.data {
      for sq in row {
        if !sq.marked {
          sum += sq.value;
        }
      }
    }
    sum
  }
}

fn part2() -> i32 {
  let (calls, mut boards) = input();
  for c in calls {
    boards.retain(|b| !b.already_won);
    let down_to_last_board = boards.len() == 1;
    for b in boards.iter_mut() {
      if let Some(score) = b.mark(c) {
        if down_to_last_board {
          return score;
        }
      }
    }
  }
  panic!("No wins");
}

fn parse_row(line: &str) -> [i32; 5] {
  line
    .split_whitespace()
    .map(|v| v.parse().unwrap())
    .collect::<Vec<i32>>()
    .try_into()
    .unwrap()
}

fn parse_board<L: Iterator>(lines: &mut L) -> Board
where
  L::Item: AsRef<str>,
{
  let mut rows: [[i32; 5]; 5] = [[0; 5]; 5];
  for i in 0..5 {
    rows[i] = parse_row(lines.next().unwrap().as_ref())
  }

  Board::new(&rows)
}

fn input() -> (Vec<i32>, Vec<Board>) {
  let mut lines = include_str!("input.txt").lines().peekable();

  // Parse first line.
  let calls = lines
    .next()
    .unwrap()
    .split(',')
    .map(|m| m.parse().unwrap())
    .collect();

  let mut boards: Vec<Board> = Vec::new();

  while lines.peek().is_some() {
    lines.next_if(|l| l.is_empty());
    boards.push(parse_board(&mut lines));
  }
  (calls, boards)
}

fn main() {
  println!("part 1: {}", part2());
}
