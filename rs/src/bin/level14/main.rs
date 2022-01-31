use std::collections::HashMap;

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

fn input() -> (&'static str, Rules) {
  let mut lines = raw_input().lines();
  let start = lines.next().unwrap();
  assert_eq!(Some(""), lines.next());
  let rules = lines.map(parse_rule).collect();
  (start, rules)
}

struct Polymer {
  first: u8,
  pairs: HashMap<(u8, u8), u64>,
}

impl Polymer {
  fn from_string(s: &str) -> Self {
    let mut pairs: HashMap<(u8, u8), u64> = HashMap::new();
    for p in s.bytes().collect::<Vec<u8>>().windows(2) {
      let pair = match p {
        [a, b] => (*a, *b),
        _ => panic!("no"),
      };
      *pairs.entry(pair).or_insert(0) += 1;
    }
    Polymer {
      first: s.bytes().next().unwrap(),
      pairs,
    }
  }

  fn grow(&mut self, rules: &Rules) {
    let mut new_pairs: HashMap<(u8, u8), u64> = HashMap::new();
    for ((l, r), count) in &self.pairs {
      let inserted_char = rules[&(*l, *r)];
      for new_pair in [(*l, inserted_char), (inserted_char, *r)] {
        *new_pairs.entry(new_pair).or_insert(0) += count;
      }
    }
    self.pairs = new_pairs;
  }

  fn frequency_map(&self) -> HashMap<u8, u64> {
    let mut freq: HashMap<u8, u64> = HashMap::new();
    for ((_, r), count) in &self.pairs {
      *freq.entry(*r).or_insert(0) += count;
    }
    *freq.entry(self.first).or_insert(0) += 1;
    freq
  }
}

fn solve(initial_state: &str, rules: &Rules) -> u64 {
  let mut p = Polymer::from_string(initial_state);
  for _ in 0..40 {
    p.grow(rules);
  }
  let freq_map = p.frequency_map();
  let most_common = freq_map.values().max().unwrap();
  let least_common = freq_map.values().min().unwrap();
  most_common - least_common
}

fn main() {
  let (initial_state, rules) = input();
  println!("{}", solve(initial_state, &rules));
}
