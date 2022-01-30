use std::collections::HashMap;
use std::iter::Peekable;

type Rule = ((u8, u8), u8);
type Rules = HashMap<(u8, u8), u8>;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn parse_rule(line: &str) -> Rule {
  let parts = line.split(" -> ").collect::<Vec<_>>();
  assert_eq!(2, parts.len());
  assert_eq!(2, parts[0].len());
  assert_eq!(1, parts[1].len());
  let mut left_side = parts[0].bytes();
  let (a, b) = (left_side.next().unwrap(), left_side.next().unwrap());
  ((a, b), parts[1].bytes().next().unwrap())
}

fn input() -> (impl Iterator<Item = u8>, Rules) {
  let mut lines = raw_input().lines();
  let start = lines.next().unwrap();
  assert_eq!(Some(""), lines.next());
  let rules = lines.map(parse_rule).collect();
  (start.bytes(), rules)
}

struct RuleApplier {
  rules: Rules,
  prev_state: Peekable<Box<dyn Iterator<Item = u8>>>,
  // The most recent char from prev_state, or none if the most recent
  // char returned was an inserted char.
  data: Option<u8>,
}

impl Iterator for RuleApplier {
  type Item = u8;
  fn next(&mut self) -> Option<u8> {
    match self.data {
      None => {
        self.data = self.prev_state.next();
        self.data
      }
      Some(c) => {
        self.data = None;
        let next_next = self.prev_state.peek();
        match next_next {
          None => None,
          Some(&nn) => Some(self.rules[&(c, nn)]),
        }
      }
    }
  }
}

fn apply(state: Box<dyn Iterator<Item = u8>>, rules: Rules) -> impl Iterator<Item = u8> {
  RuleApplier {
    rules: rules.clone(),
    prev_state: state.peekable(),
    data: None,
  }
}

fn get_answer(state: impl Iterator<Item = u8>) -> i64 {
  let mut freq_map = HashMap::new();
  for ch in state {
    *freq_map.entry(ch).or_insert(0) += 1;
  }
  let most_common = freq_map.values().max().unwrap();
  let least_common = freq_map.values().min().unwrap();
  most_common - least_common
}

fn solve(initial_state: Box<dyn Iterator<Item = u8>>, rules: Rules) -> i64 {
  let mut state = initial_state;
  for _ in 0..20 {
    state = Box::new(apply(state, rules.clone()));
  }
  get_answer(state)
}

fn main() {
  let (initial_state, rules) = input();
  println!("{}", solve(Box::new(initial_state), rules));
}
