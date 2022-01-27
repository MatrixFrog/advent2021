use advent::grid_from_input;
use advent::surrounding_coordinates;
use advent::Grid;
use std::collections::HashSet;

struct Floor {
  data: Grid<u32>,
}

impl Floor {
  fn get(&self, p: (usize, usize)) -> Option<u32> {
    self.data.get(p).copied()
  }

  fn positions(&self) -> impl std::iter::Iterator<Item = (usize, usize)> {
    self.data.positions()
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
          if self.get(neighbor).unwrap() != 9 && !basin.contains(&neighbor) {
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
    self.get(p).unwrap() + 1
  }

  fn solve_part1(&self) -> u32 {
    self.low_points().map(|p| self.risk_level(p)).sum()
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
  Floor {
    data: grid_from_input(raw_input()),
  }
}

fn main() {
  let f = input();
  println!("{}", f.solve_part1());
  println!("{}", f.solve_part2());
}
