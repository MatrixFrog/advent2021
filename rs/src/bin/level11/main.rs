use advent::positions;
use grid::*;
use std::fmt::*;

// Get the coordinates of squares adjacent to the given one (including diagonally)
pub fn surrounding_coordinates((r, c): (usize, usize)) -> Vec<(usize, usize)> {
  let mut sc = vec![(r + 1, c), (r, c + 1), (r + 1, c + 1)];
  match (r, c) {
    (0, 0) => {}
    (0, _) => {
      sc.extend([(r + 1, c - 1), (r, c - 1)]);
    }
    (_, 0) => {
      sc.extend([(r - 1, c), (r - 1, c + 1)]);
    }
    _ => sc.extend([
      (r - 1, c - 1),
      (r - 1, c),
      (r - 1, c + 1),
      (r, c - 1),
      (r + 1, c - 1),
    ]),
  }
  sc
}

struct Octo {
  value: u32,
  already_flashed: bool,
}

impl Debug for Octo {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.value)
  }
}

impl Octo {
  fn new(value: u32) -> Self {
    Self {
      value,
      already_flashed: false,
    }
  }

  // Increment the value of this Octo, returning true if this
  // caused a flash
  fn inc(&mut self) -> bool {
    self.value += 1;
    if !self.already_flashed && self.value > 9 {
      self.already_flashed = true;
      return true;
    }
    false
  }

  fn end_step(&mut self) {
    if self.already_flashed {
      self.value = 0;
      self.already_flashed = false;
    }
  }
}

struct Octos {
  grid: Grid<Octo>,
  flashes: usize,
}

impl Debug for Octos {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.grid)
  }
}

impl Octos {
  fn new(grid: Grid<Octo>) -> Self {
    Self { grid, flashes: 0 }
  }
  fn len(&self) -> usize {
    self.grid.flatten().len()
  }
  fn step(&mut self) -> usize {
    let mut to_increase = positions(&self.grid).collect::<Vec<_>>();
    while !to_increase.is_empty() {
      let mut to_increase_next = vec![];
      for (r, c) in to_increase {
        let flashed = self.grid.get_mut(r, c).map_or(false, |o| o.inc());
        if flashed {
          to_increase_next.extend(surrounding_coordinates((r, c)))
        }
      }
      to_increase = to_increase_next;
    }
    let flashes_this_step = self.grid.iter().filter(|o| o.already_flashed).count();
    self.flashes += flashes_this_step;
    for o in self.grid.iter_mut() {
      o.end_step();
    }
    flashes_this_step
  }
}

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn parse_input(input: &str) -> Octos {
  let mut grid = grid![];
  for l in input.lines() {
    grid.push_row(
      l.trim()
        .chars()
        .map(|c| Octo::new(c.to_digit(10).unwrap()))
        .collect(),
    );
  }
  Octos::new(grid)
}

fn _minigrid() -> Octos {
  let grid: Grid<u32> = grid![
    [1, 1, 1, 1, 1]
    [1, 9, 9, 9, 1]
    [1, 9, 1, 9, 1]
    [1, 9, 9, 9, 1]
    [1, 1, 1, 1, 1]
  ];

  Octos::new(Grid::from_vec(
    grid.iter().map(|v| Octo::new(*v)).collect(),
    5,
  ))
}

fn main() {
  let mut octos = parse_input(raw_input());
  // let mut octos = minigrid();
  for i in 1.. {
    let flashes = octos.step();
    if flashes == octos.len() {
      println!("all flashed: {}", i);
      break;
    }
  }
}
