use grid::Grid;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

#[derive(Debug)]
enum FoldType {
  X,
  Y,
}
#[derive(Debug)]
struct Fold {
  fold_type: FoldType,
  value: usize,
}

fn parse_coord_line(line: &str) -> (usize, usize) {
  let coords = line
    .split(',')
    .map(|c| c.parse::<usize>().unwrap())
    .collect::<Vec<_>>();
  assert_eq!(2, coords.len());
  (coords[0], coords[1])
}

fn build_grid(points: &[(usize, usize)]) -> Grid<bool> {
  let columns = *points.iter().map(|(x, _)| x).max().unwrap() + 1;
  let rows = *points.iter().map(|(_, y)| y).max().unwrap() + 1;
  let mut g = Grid::new(rows, columns);
  for (x, y) in points {
    *g.get_mut(*y, *x).unwrap() = true;
  }
  g
}

fn parse_fold(line: &str) -> Fold {
  assert!(line.starts_with("fold along "));
  let important_part = line.split_whitespace().last().unwrap();
  let parts = important_part.split('=').collect::<Vec<&str>>();
  assert_eq!(2, parts.len());
  let fold_type = match parts[0] {
    "x" => FoldType::X,
    "y" => FoldType::Y,
    _ => panic!(),
  };
  let value = parts[1].parse::<usize>().unwrap();
  Fold { fold_type, value }
}

fn input() -> (Grid<bool>, Vec<Fold>) {
  let mut points = vec![];
  let mut lines = raw_input().lines();
  loop {
    let line = lines.next().unwrap().trim();
    if line.is_empty() {
      break;
    }
    points.push(parse_coord_line(line));
  }
  let mut folds = vec![];
  for line in lines {
    folds.push(parse_fold(line))
  }
  (build_grid(&points), folds)
}

fn apply_fold(g: &mut Grid<bool>, fold: &Fold) {
  match fold {
    Fold {
      fold_type: X,
      value,
    } => {
      let mut x = 1;
      loop {
        if value + x == g.cols() {
          break;
        }
        for (row, sq) in g.iter_col(value + x).enumerate() {
          *g.get_mut(row, value - x).unwrap() |= sq;
        }
        x += 1;
      }
    }
    _ => panic!("not implemented"),
  }
}

fn main() {
  let (grid, folds) = input();
  println!("{:?}", folds[0]);
}
