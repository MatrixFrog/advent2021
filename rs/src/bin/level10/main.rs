fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> impl Iterator<Item = &'static str> {
  raw_input().lines()
}

#[derive(Debug)]
enum ParseResult {
  OK, // apparently never happens
  Incomplete(Vec<char>),
  Corrupt(char),
}

struct Parser {
  stack: Vec<char>,
}

impl Parser {
  fn new() -> Self {
    Parser { stack: vec![] }
  }

  fn check(&self, ch: char) -> bool {
    match self.stack.last() {
      None => panic!(),
      Some(last) => match last {
        '(' => ch == ')',
        '[' => ch == ']',
        '{' => ch == '}',
        '<' => ch == '>',
        _ => panic!(),
      },
    }
  }

  fn try_pop(&mut self, ch: char) -> Result<(), char> {
    if self.check(ch) {
      self.stack.pop();
      Result::Ok(())
    } else {
      Result::Err(ch)
    }
  }

  fn handle_char(&mut self, ch: char) -> Result<(), char> {
    match ch {
      '(' | '[' | '{' | '<' => {
        self.stack.push(ch);
        Result::Ok(())
      }
      ')' | ']' | '}' | '>' => self.try_pop(ch),
      _ => panic!("Unexpected char {}", ch),
    }
  }

  fn parse_line(&mut self, line: &str) -> ParseResult {
    for ch in line.chars() {
      match self.handle_char(ch) {
        Ok(_) => {}
        Err(err_ch) => {
          return ParseResult::Corrupt(err_ch);
        }
      }
    }
    if self.stack.is_empty() {
      ParseResult::OK
    } else {
      ParseResult::Incomplete(self.stack.clone())
    }
  }
}

fn parse_line(line: &str) -> ParseResult {
  let mut p = Parser::new();
  p.parse_line(line)
}

fn parse_all() -> Vec<ParseResult> {
  input().map(parse_line).collect()
}

fn part1(results: &[ParseResult]) -> i32 {
  let mut total = 0;
  for r in results {
    if let ParseResult::Corrupt(ch) = r {
      total += match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
      }
    }
  }
  total
}

fn val(ch: char) -> i64 {
  match ch {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4,
    _ => panic!(),
  }
}

fn score(stack: &[char]) -> i64 {
  let mut score: i64 = 0;
  for ch in stack.iter().rev() {
    score = score * 5 + val(*ch);
  }
  score
}

fn scores(results: &[ParseResult]) -> Vec<i64> {
  let mut scores = vec![];
  // Instead of filling in the )]}>'s, just look at what's left on the stack, in reverse order
  for r in results {
    if let ParseResult::Incomplete(stack) = r {
      scores.push(score(stack))
    }
  }
  scores
}

fn part2(results: &[ParseResult]) -> i64 {
  let mut scores = scores(results);
  assert_eq!(scores.len() % 2, 1);
  scores.sort_unstable();
  *scores.get(scores.len() / 2).unwrap()
}

fn main() {
  let results = parse_all();
  println!("part 1: {}", part1(&results));
  println!("part 1: {}", part2(&results));
}
