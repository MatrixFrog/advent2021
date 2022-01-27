use itertools::Itertools;

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
// A rectangular grid of data, addressable by (row, column) pairs
pub struct Grid<T> {
  data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
  pub fn new(data: Vec<Vec<T>>) -> Self {
    assert!(data.windows(2).all(|w| w[0].len() == w[1].len()));
    Self { data }
  }

  // Get the value in row 'r' / column 'c'
  pub fn get(&self, (r, c): (usize, usize)) -> Option<&T> {
    self.data.get(r).and_then(|row| row.get(c))
  }

  // Get the coordinates of the squares that neighbor 'p'
  pub fn neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
    surrounding_coordinates(p)
      .into_iter()
      .filter(|&sc| self.get(sc).is_some())
      .collect::<Vec<(usize, usize)>>()
  }

  pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
    (0..self.data.len()).cartesian_product(0..self.data[0].len())
  }
}
