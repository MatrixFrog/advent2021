use std::collections::hash_map::HashMap;

fn input() -> Vec<u8> {
  let mut fish = Vec::new();
  for l in include_str!("input.txt").split(',') {
    fish.push(l.trim().parse().unwrap());
  }
  fish
}

fn input_as_map() -> HashMap<u8, u64> {
  let mut aquarium: HashMap<u8, u64> = HashMap::new();
  for f in input() {
    *aquarium.entry(f).or_default() += 1;
  }
  aquarium
}

fn tick(aquarium: &HashMap<u8, u64>) -> HashMap<u8, u64> {
  let mut new_aquarium: HashMap<u8, u64> = HashMap::new();
  for i in 0..9 {
    new_aquarium.insert(i, *aquarium.get(&(i + 1)).unwrap_or(&0));
  }
  new_aquarium.insert(
    6,
    new_aquarium.get(&6).unwrap_or(&0) + aquarium.get(&0).unwrap_or(&0),
  );
  new_aquarium.insert(8, *aquarium.get(&0).unwrap_or(&0));
  new_aquarium
}

fn solve() -> (u64, u64) {
  let mut part1 = 0;
  let mut aquarium = input_as_map();
  for i in 0..256 {
    aquarium = tick(&aquarium);
    if i == 79 {
      part1 = aquarium.values().sum::<u64>();
    }
  }

  let part2 = aquarium.values().sum::<u64>();
  (part1, part2)
}

fn main() {
  println!("{:?}", solve());
}
