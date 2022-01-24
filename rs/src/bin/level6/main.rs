#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Fish {
  value: u8,
}

fn raw_input() -> &'static str {
  include_str!("input.txt")
  // "3,4,3,1,2"
}

fn input() -> Vec<Fish> {
  let mut fish = Vec::new();
  for l in raw_input().split(',') {
    fish.push(Fish {
      value: l.trim().parse().unwrap(),
    })
  }
  fish
}

fn tick(fish: &mut Vec<Fish>) {
  let mut new_fish = 0;
  for f in fish.iter_mut() {
    if f.value == 0 {
      f.value = 6;
      new_fish += 1;
    } else {
      f.value -= 1;
    }
  }
  for _ in 0..new_fish {
    fish.push(Fish { value: 8 });
  }
}

fn solve(turns: i32) -> usize {
  let mut fish = input();
  for _ in 0..turns {
    tick(&mut fish)
  }

  fish.len()
}

fn main() {
  println!("part 1: {}", solve(80));
  println!("part 2: {}", solve(256));
}
