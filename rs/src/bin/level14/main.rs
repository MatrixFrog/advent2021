use std::collections::HashMap;
use std::iter::Peekable;

type Rule = ((char, char), char);
type Rules = HashMap<(char, char), char>;
type State = dyn Iterator<Item = char>;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn parse_rule(line: &str) -> Rule {
  let parts = line.split(" -> ").collect::<Vec<_>>();
  assert_eq!(2, parts.len());
  assert_eq!(2, parts[0].len());
  assert_eq!(1, parts[1].len());
  let mut left_side = parts[0].chars();
  let (a, b) = (left_side.next().unwrap(), left_side.next().unwrap());
  ((a, b), parts[1].chars().next().unwrap())
}

fn input() -> (impl Iterator<Item = char>, Rules) {
  let mut lines = raw_input().lines();
  let start = lines.next().unwrap();
  assert_eq!(Some(""), lines.next());
  let rules = lines.map(parse_rule).collect();
  (start.chars(), rules)
}

struct RuleApplier {
  rules: Rules,
  prev_state: Peekable<Box<State>>,
  // The most recent char from prev_state, or none if the most recent
  // char returned was an inserted char.
  data: Option<char>,
}

impl Iterator for RuleApplier {
  type Item = char;
  fn next(&mut self) -> Option<char> {
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

fn apply(state: Box<State>, rules: Rules) -> impl Iterator<Item = char> {
  RuleApplier {
    rules: rules.clone(),
    prev_state: state.peekable(),
    data: None,
  }
}

fn get_answer(state: impl Iterator<Item = char>) -> i64 {
  let mut freq_map = HashMap::new();
  for ch in state {
    *freq_map.entry(ch).or_insert(0) += 1;
  }
  let most_common = freq_map.values().max().unwrap();
  let least_common = freq_map.values().min().unwrap();
  most_common - least_common
}

fn solve(initial_state: Box<State>, rules: Rules) -> i64 {
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
