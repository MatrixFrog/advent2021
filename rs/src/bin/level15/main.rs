use advent::grid_from_input;
use grid::*;
use permute::*;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> Grid<u32> {
  grid_from_input(raw_input())
}

#[derive(Clone, Copy)]
enum Step {
  Right,
  Down,
}

type Path = Vec<Step>;
// type RefPath<'a> = Vec<&'a Step>;

fn total_risk(path: &Path, grid: &Grid<u32>) -> u32 {
  let mut x = 0;
  let mut y = 0;
  let mut total = 0;
  for step in path {
    match step {
      Step::Right => x += 1,
      Step::Down => y += 1,
    }
    total += grid[y][x];
  }
  total
}

fn all_paths_to(point: (usize, usize)) -> Vec<Path> {
  let (x, y) = point;
  let mut one_path = vec![Step::Right; x];
  one_path.append(&mut vec![Step::Down; y]);
  permute(one_path)
}

fn min_risk(point: (usize, usize), grid: &Grid<u32>) -> u32 {
  all_paths_to(point)
    .iter()
    .map(|path| total_risk(path, grid))
    .min()
    .unwrap()
}

fn main() {
  let grid = input();
  let goal = (grid.cols() - 1, grid.rows() - 1);

  println!("{}", min_risk(goal, &grid))
}
