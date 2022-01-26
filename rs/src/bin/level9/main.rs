use itertools::Itertools;
use std::collections::HashSet;

struct Floor {
  data: Vec<Vec<u32>>,
}

fn surrounding_coordinates((r, c): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
  let mut sc = vec![(r + 1, c), (r, c + 1)];
  if r != 0 {
    sc.push((r - 1, c))
  }
  if c != 0 {
    sc.push((r, c - 1))
  }
  sc.into_iter()
}

impl Floor {
  fn get(&self, (r, c): (usize, usize)) -> Option<&u32> {
    self.data.get(r).and_then(|row| row.get(c))
  }

  fn neighbors(&self, p: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
    surrounding_coordinates(p).filter(|sc| self.get(*sc).is_some())
  }

  fn is_low_point(&self, p: (usize, usize)) -> bool {
    let v = self.get(p).unwrap();
    for neighbor in self.neighbors(p) {
      if v < self.get(neighbor).unwrap() {
        // ok
      } else {
        return false;
      }
    }
    true
  }

  fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
    self.positions().filter(|p| self.is_low_point(*p))
  }

  // Given a low point 'p', find the size of the basin for p.
  fn basin(&self, p: (usize, usize)) -> usize {
    let mut basin = HashSet::new();
    basin.insert(p);
    // TODO...
    basin.len()
  }

  fn risk_level(&self, p: (usize, usize)) -> u32 {
    if self.is_low_point(p) {
      self.get(p).unwrap() + 1
    } else {
      0
    }
  }

  fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
    (0..self.data.len()).cartesian_product(0..self.data[0].len())
  }

  fn solve_part1(&self) -> u32 {
    self.positions().map(|p| self.risk_level(p)).sum()
  }

  fn solve_part2(&self) -> u32 {
    // TODO...
    0
  }
}

// fn raw_input() -> &'static str {
//   include_str!("input.txt")
// }

fn raw_input() -> &'static str {
  "2199943210
3987894921
9856789892
8767896789
9899965678"
}

fn input() -> Floor {
  let mut data = Vec::new();
  for l in raw_input().lines() {
    data.push(
      l.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>(),
    )
  }
  Floor { data }
}

fn main() {
  let f = input();
  println!("{}", f.solve_part1());
  println!("{}", f.solve_part2());
}
