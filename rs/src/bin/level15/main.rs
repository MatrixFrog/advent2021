use advent::{grid_from_input, positions};
use grid::*;
use std::cmp::min;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> Grid<u32> {
  grid_from_input(raw_input())
}

type MinRiskGrid = Grid<Option<u32>>;

fn min_risk(pos: (usize, usize), min_risk_grid: &mut MinRiskGrid, grid: &Grid<u32>) {
  let (x, y) = pos;
  if y == 0 {
    min_risk_grid[y][x] = Some((1..=x).map(|xx| grid[y][xx]).sum());
  } else if x == 0 {
    min_risk_grid[y][x] = Some((1..=y).map(|yy| grid[yy][x]).sum())
  } else {
    match min_risk_grid[y][x - 1] {
      Some(min_risk_to_left) => match min_risk_grid[y - 1][x] {
        Some(min_risk_above) => {
          min_risk_grid[y][x] = Some(min(min_risk_above, min_risk_to_left) + grid[y][x]);
        }
        None => panic!(),
      },
      None => panic!(),
    }
  }
}

fn make_min_risk_grid(grid: &Grid<u32>) -> MinRiskGrid {
  let mut min_risk_grid = Grid::new(grid.rows(), grid.cols());
  for pos in positions(grid) {
    min_risk(pos, &mut min_risk_grid, grid)
  }
  min_risk_grid
}

fn main() {
  let grid = input();
  assert_eq!(grid.rows(), grid.cols());
  let min_risk_grid: MinRiskGrid = make_min_risk_grid(&grid);

  println!(
    "{}",
    min_risk_grid[grid.cols() - 1][grid.rows() - 1].unwrap()
  )
}
