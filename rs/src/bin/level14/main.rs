use itertools::Itertools;
use std::collections::HashMap;

type Rule = ((char, char), char);

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

fn input() -> (Vec<char>, Vec<Rule>) {
  let mut lines = raw_input().lines();
  let start = lines.next().unwrap();
  assert_eq!(Some(""), lines.next());
  let rules = lines.map(parse_rule).collect();
  (start.chars().collect(), rules)
}

fn apply(state: &mut Vec<char>, rules: &[Rule]) {
  let mut insertions = vec![];
  for w in state.windows(2) {
    match w {
      [a, b] => {
        for ((ra, rb), r) in rules {
          if (ra, rb) == (a, b) {
            insertions.push(r);
            break;
          }
        }
      }
      _ => panic!("nope"),
    }
  }
  *state = state.iter().interleave(insertions).copied().collect();
}

fn get_answer(state: &[char]) -> i32 {
  let mut freq_map = HashMap::new();
  for ch in state {
    *freq_map.entry(ch).or_insert(0) += 1;
  }
  let most_common = freq_map.values().max().unwrap();
  let least_common = freq_map.values().min().unwrap();
  most_common - least_common
}

fn main() {
  let (mut state, rules) = input();
  for _ in 0..10 {
    // println!("{:?}", state);
    apply(&mut state, &rules);
  }
  println!("{}", get_answer(&state))
}
