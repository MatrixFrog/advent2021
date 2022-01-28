use std::collections::HashSet;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

type Point = (usize, usize);

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

fn parse_coord_line(line: &str) -> Point {
  let coords = line
    .split(',')
    .map(|c| c.parse::<usize>().unwrap())
    .collect::<Vec<_>>();
  assert_eq!(2, coords.len());
  (coords[0], coords[1])
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

fn input() -> (HashSet<Point>, Vec<Fold>) {
  let mut points = HashSet::new();
  let mut lines = raw_input().lines();
  loop {
    let line = lines.next().unwrap().trim();
    if line.is_empty() {
      break;
    }
    points.insert(parse_coord_line(line));
  }
  let mut folds = vec![];
  for line in lines {
    folds.push(parse_fold(line))
  }
  (points, folds)
}

fn apply_fold(points: &mut HashSet<Point>, fold: &Fold) {
  match fold {
    Fold {
      fold_type: FoldType::X,
      value,
    } => {
      let mut new_dots: Vec<Point> = vec![];
      let mut old_dots: Vec<Point> = vec![];
      for (x, y) in points.iter() {
        if x < value { /* no effect */
        } else {
          old_dots.push((*x, *y));
          new_dots.push((value - (x - value), *y));
        }
      }
      points.extend(new_dots);
      for d in old_dots {
        points.remove(&d);
      }
    }
    Fold {
      fold_type: FoldType::Y,
      value,
    } => {
      let mut new_dots: Vec<Point> = vec![];
      let mut old_dots: Vec<Point> = vec![];
      for (x, y) in points.iter() {
        if y < value { /* no effect */
        } else {
          old_dots.push((*x, *y));
          new_dots.push((*x, value - (y - value)));
        }
      }
      points.extend(new_dots);
      for d in old_dots {
        points.remove(&d);
      }
    }
  }
}

fn print_grid(points: &HashSet<Point>) {
  let width = *points.iter().map(|(x, _)| x).max().unwrap();
  let height = *points.iter().map(|(_, y)| y).max().unwrap();
  for y in 0..=height {
    for x in 0..=width {
      if points.contains(&(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
}

fn main() {
  let (mut points, folds) = input();
  for f in folds {
    apply_fold(&mut points, &f);
  }
  print_grid(&points);
}
