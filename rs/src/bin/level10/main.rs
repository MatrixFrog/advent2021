fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn input() -> impl Iterator<Item = &'static str> {
  raw_input().lines()
}

#[derive(Debug)]
enum ParseResult {
  OK, // apparently never happens
  Incomplete,
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
      ParseResult::Incomplete
    } else {
      ParseResult::OK
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

fn part2(results: &[ParseResult]) -> i32 {
  panic!("TODO")
}

fn main() {
  let results = parse_all();
  println!("part 1: {}", part1(&results));
  println!("part 1: {}", part2(&results));
}
