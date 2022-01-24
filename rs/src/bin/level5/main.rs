use std::collections::hash_map::HashMap;
use std::fmt;

#[derive(Hash, Eq, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_tuple("").field(&self.x).field(&self.y).finish()
  }
}

#[derive(Debug)]
struct Line {
  start: Point,
  end: Point,
}

impl Line {
  // Check if it's a horizontal/vertical line.
  fn is_orthogonal(&self) -> bool {
    self.start.x == self.end.x || self.start.y == self.end.y
  }

  fn points(&self) -> Vec<Point> {
    let mut pts = Vec::new();
    if self.start.x == self.end.x {
      for y in range(self.start.y, self.end.y) {
        pts.push(Point { x: self.start.x, y });
      }
    } else if self.start.y == self.end.y {
      for x in range(self.start.x, self.end.x) {
        pts.push(Point { x, y: self.start.y })
      }
    } else {
      panic!("Can't get points for non-ortho line");
    }
    pts
  }
}

fn range(a: i32, b: i32) -> Vec<i32> {
  let mut r = Vec::new();
  if a <= b {
    for i in a..b + 1 {
      r.push(i);
    }
  } else {
    for i in b..a + 1 {
      r.push(i);
    }
  }
  r
}

fn part1() -> usize {
  let mut lines = input();
  lines.retain(|l| l.is_orthogonal());
  let mut world: HashMap<Point, i32> = HashMap::new();
  for l in lines {
    for p in l.points() {
      let count = world.entry(p).or_default();
      *count += 1;
    }
  }
  world.values().filter(|v| **v >= 2).count()
}

fn parse_point(s: &str) -> Point {
  let values = s.split(',').collect::<Vec<&str>>();
  assert_eq!(2, values.len());
  Point {
    x: values[0].parse().unwrap(),
    y: values[1].parse().unwrap(),
  }
}

fn parse_line(s: &str) -> Line {
  let values = s.split(" -> ").collect::<Vec<&str>>();
  assert_eq!(2, values.len());
  Line {
    start: parse_point(values[0]),
    end: parse_point(values[1]),
  }
}

fn raw_input() -> &'static str {
  include_str!("input.txt")
  //   "0,9 -> 5,9
  // 8,0 -> 0,8
  // 9,4 -> 3,4
  // 2,2 -> 2,1
  // 7,0 -> 7,4
  // 6,4 -> 2,0
  // 0,9 -> 2,9
  // 3,4 -> 1,4
  // 0,0 -> 8,8
  // 5,5 -> 8,2"
}

fn input() -> Vec<Line> {
  let mut lines = Vec::new();
  for l in raw_input().lines() {
    lines.push(parse_line(l))
  }
  lines
}

fn main() {
  println!("part 1: {}", part1());
}
