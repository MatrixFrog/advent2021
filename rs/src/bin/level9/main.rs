use itertools::Itertools;
use std::collections::HashSet;

struct Floor {
  data: Vec<Vec<u32>>,
}

fn surrounding_coordinates((r, c): (usize, usize)) -> Vec<(usize, usize)> {
  let mut sc = vec![(r + 1, c), (r, c + 1)];
  if r != 0 {
    sc.push((r - 1, c))
  }
  if c != 0 {
    sc.push((r, c - 1))
  }
  sc
}

impl Floor {
  fn get(&self, (r, c): (usize, usize)) -> Option<&u32> {
    self.data.get(r).and_then(|row| row.get(c))
  }

  fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
    surrounding_coordinates(p)
      .into_iter()
      .filter(|&sc| self.get(sc).is_some())
      .collect::<Vec<(usize, usize)>>()
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
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    let mut additions = vec![p];
    loop {
      basin.extend(&additions);
      let mut new_additions = vec![];
      for new_pt in &additions {
        for neighbor in self.neighbors(*new_pt) {
          if *self.get(neighbor).unwrap() != 9 && !basin.contains(&neighbor) {
            new_additions.push(neighbor)
          }
        }
      }
      if new_additions.is_empty() {
        break;
      }
      additions = new_additions;
    }
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

  fn solve_part2(&self) -> usize {
    let mut basins = self
      .low_points()
      .map(|lp| self.basin(lp))
      .collect::<Vec<usize>>();
    basins.sort_unstable();
    basins.reverse();
    basins.iter().take(3).product()
  }
}

fn raw_input() -> &'static str {
  include_str!("input.txt")
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
