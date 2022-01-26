use itertools::Itertools;

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

  fn low_point(&self, p: (usize, usize)) -> bool {
    let v = self.get(p).unwrap();
    for neighbor in surrounding_coordinates(p).map(|sc| self.get(sc)).flatten() {
      if v < neighbor {
        // ok
      } else {
        return false;
      }
    }
    true
  }

  fn risk_level(&self, p: (usize, usize)) -> u32 {
    if self.low_point(p) {
      self.get(p).unwrap() + 1
    } else {
      0
    }
  }

  fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
    (0..self.data.len()).cartesian_product(0..self.data[0].len())
  }

  fn solve(&self) -> u32 {
    self.positions().map(|p| self.risk_level(p)).sum()
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
  println!("{}", f.solve())
}
