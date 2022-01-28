use grid::*;
use itertools::Itertools;

pub fn positions<T>(grid: &Grid<T>) -> impl std::iter::Iterator<Item = (usize, usize)> {
  (0..grid.rows()).cartesian_product(0..grid.cols())
}

struct Splitter {
  v: Vec<Option<char>>,
  input: Box<dyn Iterator<Item = char>>,
}

impl Splitter {
  fn new(input: Box<dyn Iterator<Item = char>>) -> Self {
    Splitter { v: vec![], input }
  }

  fn make_iter(&mut self) -> I {
    I { s: self, i: 0 }
  }

  fn get(&mut self, i: usize) -> Option<char> {
    while i >= self.v.len() {
      self.v.push(self.input.next());
    }
    self.v[i]
  }
}

struct I<'a> {
  s: &'a mut Splitter,
  i: usize,
}

impl<'a> Iterator for I<'a> {
  type Item = char;
  fn next(&mut self) -> Option<char> {
    let val = self.s.get(self.i);
    self.i += 1;
    val
  }
}

fn duplicate_iterator(input: impl Iterator<Item = char> + 'static) -> (I<'static>, I<'static>) {
  let mut s = Splitter::new(Box::new(input));
  (s.make_iter(), s.make_iter())
}
