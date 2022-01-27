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
  Corrupt(Vec<char>),
}

struct Parser {
  stack: Vec<char>,
  errors: Vec<char>,
}

impl Parser {
  fn new() -> Self {
    Parser {
      stack: vec![],
      errors: vec![],
    }
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

  fn try_pop(&mut self, ch: char) {
    if self.check(ch) {
      self.stack.pop();
    } else {
      self.errors.push(ch);
    }
  }

  fn handle_char(&mut self, ch: char) {
    match ch {
      '(' | '[' | '{' | '<' => self.stack.push(ch),
      ')' | ']' | '}' | '>' => self.try_pop(ch),
      _ => panic!("Unexpected char {}", ch),
    }
  }

  fn parse_line(&mut self, line: &str) -> ParseResult {
    for ch in line.chars() {
      self.handle_char(ch);
    }
    if !self.errors.is_empty() {
      ParseResult::Corrupt(self.errors.clone())
    } else if !self.stack.is_empty() {
      ParseResult::Incomplete
    } else {
      panic!()
    }
  }
}

fn parse_line(line: &str) -> ParseResult {
  let mut p = Parser::new();
  p.parse_line(line)
}

fn main() {
  for l in input() {
    println!("{:?}", parse_line(l));
  }
}
