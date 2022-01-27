use grid::*;

// Get the coordinates of squares adjacent to the given one (not including diagonally)
pub fn surrounding_coordinates((r, c): (usize, usize)) -> Vec<(usize, usize)> {
  let mut sc = vec![(r + 1, c), (r, c + 1)];
  if r != 0 {
    sc.push((r - 1, c))
  }
  if c != 0 {
    sc.push((r, c - 1))
  }
  sc
}

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
