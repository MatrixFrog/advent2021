use grid::*;
use itertools::Itertools;

pub fn positions<T>(grid: &Grid<T>) -> impl std::iter::Iterator<Item = (usize, usize)> {
  (0..grid.rows()).cartesian_product(0..grid.cols())
}

// Parse a grid of digits. First used in level 9.
pub fn grid_from_input(input: &str) -> Grid<u32> {
  let mut g = grid![];
  for l in input.lines() {
    g.push_row(
      l.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>(),
    );
  }
  g
}
