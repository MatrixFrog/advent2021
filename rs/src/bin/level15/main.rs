use advent::grid_from_input;
use grid::*;
fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> Grid<u32> {
  grid_from_input(raw_input())
}
fn main() {
  let g = input();
  println!("{:?}", g);
}
