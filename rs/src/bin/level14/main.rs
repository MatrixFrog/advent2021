use nohash::BuildNoHashHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::Peekable;

type Rule = (CharPair, u8);
type Rules = HashMap<CharPair, u8, BuildNoHashHasher<CharPair>>;

#[derive(Eq, Clone)]
struct CharPair((u8, u8));

impl PartialEq for CharPair {
  fn eq(&self, other: &CharPair) -> bool {
    self.0 == other.0
  }
}

impl Hash for CharPair {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    let (a, b) = self.0;
    hasher.write_u16(((a as u16) << 8) + (b as u16));
  }
}
impl nohash::IsEnabled for CharPair {}

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
  (CharPair((a, b)), parts[1].bytes().next().unwrap())
}

fn input() -> (impl Iterator<Item = u8>, Rules) {
  let mut lines = raw_input().lines();
  let start = lines.next().unwrap();
  assert_eq!(Some(""), lines.next());
  let rules = lines.map(parse_rule);
  let mut rule_map = HashMap::with_hasher(BuildNoHashHasher::default());
  rule_map.extend(rules);
  (start.bytes(), rule_map)
}

struct RuleApplier<'a> {
  rules: &'a Rules,
  prev_state: Peekable<Box<dyn Iterator<Item = u8> + 'a>>,
  // The most recent char from prev_state, or none if the most recent
  // char returned was an inserted char.
  data: Option<u8>,
}

impl<'a> Iterator for RuleApplier<'a> {
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
          Some(&nn) => Some(self.rules[&CharPair((c, nn))]),
        }
      }
    }
  }
}

fn apply<'a>(
  state: Box<dyn Iterator<Item = u8> + 'a>,
  rules: &'a Rules,
) -> impl Iterator<Item = u8> + 'a {
  RuleApplier {
    rules,
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

fn solve<'a>(initial_state: Box<dyn Iterator<Item = u8> + 'a>, rules: &'a Rules) -> i64 {
  let mut state = initial_state;
  for _ in 0..20 {
    state = Box::new(apply(state, rules));
  }
  get_answer(state)
}

fn main() {
  let (initial_state, rules) = input();
  println!("{}", solve(Box::new(initial_state), &rules));
}
