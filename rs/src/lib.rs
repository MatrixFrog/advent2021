use grid::*;
use itertools::Itertools;

pub fn positions<T>(grid: &Grid<T>) -> impl std::iter::Iterator<Item = (usize, usize)> {
  (0..grid.rows()).cartesian_product(0..grid.cols())
}
