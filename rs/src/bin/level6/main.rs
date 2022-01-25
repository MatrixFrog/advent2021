use std::collections::hash_map::HashMap;

fn raw_input() -> &'static str {
  include_str!("input.txt")
  // "3,4,3,1,2"
}

fn input() -> Vec<u8> {
  let mut fish = Vec::new();
  for l in raw_input().split(',') {
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

fn tick(aquarium: &HashMap<u8, u64>)->HashMap<u8, u64> {
  let mut new_aquarium: HashMap<u8, u64> = HashMap::new();
  for i in 0..9 {
    new_aquarium.insert(i, *aquarium.get(&(i+1)).unwrap_or(&0));
  }
  new_aquarium.insert(6, new_aquarium.get(&6).unwrap_or(&0) +  aquarium.get(&0).unwrap_or(&0));
  new_aquarium.insert(8, *aquarium.get(&0).unwrap_or(&0));
  new_aquarium
}

fn main() {
  let turns = 256;
  let mut aquarium = input_as_map();
  for _ in 0..turns {
    aquarium = tick(&aquarium);
  }
  println!("{:?}", aquarium.values().sum::<u64>());
}
