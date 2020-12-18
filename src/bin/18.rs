use common::{Day, Part};
use std::collections::VecDeque;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-18-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(0, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-18-input.txt");
    std::process::exit(1);
  }
}

#[derive(Debug, PartialEq)]
enum Token {
  Number(i32),
  Plus,
  Times,
  OpenParen,
  CloseParen,
}

pub fn part_1(_data: &[&str]) -> u64 {
  0
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn evaluate_infix(infix: &VecDeque<Token>) -> i32 {
  evaluate_postfix(&infix_to_postfix(infix))
}

fn evaluate_postfix(postfix: &VecDeque<Token>) -> i32 {
  let mut stack: VecDeque<i32> = VecDeque::new();

  for token in postfix.iter() {
    match token {
      &Token::Number(number) => stack.push_front(number),
      &Token::Plus => {
        let x = stack.pop_front().unwrap();
        let y = stack.pop_front().unwrap();
        stack.push_front(x + y);
      }
      &Token::Times => {
        let x = stack.pop_front().unwrap();
        let y = stack.pop_front().unwrap();
        stack.push_front(x * y);
      }
      &Token::OpenParen => {}
      &Token::CloseParen => {}
    }
  }

  stack.pop_front().unwrap()
}

fn infix_to_postfix(infix: &VecDeque<Token>) -> VecDeque<Token> {
  let mut postfix: VecDeque<Token> = VecDeque::with_capacity(infix.capacity());
  let mut stack: VecDeque<Token> = VecDeque::new();

  for token in infix.iter() {
    match token {
      &Token::Number(number) => postfix.push_back(Token::Number(number)),
      &Token::OpenParen => stack.push_front(Token::OpenParen),
      &Token::CloseParen => {
        while stack.front() != Some(&Token::OpenParen) {
          let op = stack.pop_front().unwrap();
          postfix.push_back(op);
        }
        stack.pop_front(); // discard the open paren
      }
      &Token::Plus => {
        while stack.len() > 0 && stack.front() != Some(&Token::OpenParen) {
          postfix.push_back(stack.pop_front().unwrap());
        }
        stack.push_front(Token::Plus);
      }
      &Token::Times => {
        while stack.len() > 0 && stack.front() != Some(&Token::OpenParen) {
          postfix.push_back(stack.pop_front().unwrap());
        }
        stack.push_front(Token::Times);
      }
    }
  }

  // drain anything remaining on the stack
  while stack.len() > 0 {
    postfix.push_back(stack.pop_front().unwrap());
  }

  postfix
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![];
    assert_eq!(part_1(&data), 0);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_evaluate() {
    assert_eq!(
      evaluate_infix(&VecDeque::from(vec![
        Token::Number(1),
        Token::Plus,
        Token::Number(2),
        Token::Times,
        Token::Number(3),
        Token::Plus,
        Token::Number(4),
        Token::Times,
        Token::Number(5),
        Token::Plus,
        Token::Number(6),
      ])),
      71
    );
    assert_eq!(
      evaluate_infix(&VecDeque::from(vec![
        Token::Number(1),
        Token::Plus,
        Token::OpenParen,
        Token::Number(2),
        Token::Times,
        Token::Number(3),
        Token::CloseParen,
        Token::Plus,
        Token::OpenParen,
        Token::Number(4),
        Token::Times,
        Token::OpenParen,
        Token::Number(5),
        Token::Plus,
        Token::Number(6),
        Token::CloseParen,
        Token::CloseParen,
      ])),
      51
    );
  }
  #[test]
  fn test_infix_to_postfix_simple() {
    let data: VecDeque<Token> = VecDeque::from(vec![
      Token::Number(1),
      Token::Plus,
      Token::Number(2),
      Token::Times,
      Token::Number(3),
      Token::Plus,
      Token::Number(4),
      Token::Times,
      Token::Number(5),
      Token::Plus,
      Token::Number(6),
    ]);
    assert_eq!(
      infix_to_postfix(&data),
      VecDeque::from(vec![
        Token::Number(1),
        Token::Number(2),
        Token::Plus,
        Token::Number(3),
        Token::Times,
        Token::Number(4),
        Token::Plus,
        Token::Number(5),
        Token::Times,
        Token::Number(6),
        Token::Plus,
      ])
    );
  }

  #[test]
  fn test_infix_to_postfix_parens() {
    let data: VecDeque<Token> = VecDeque::from(vec![
      Token::Number(1),
      Token::Plus,
      Token::OpenParen,
      Token::Number(2),
      Token::Times,
      Token::Number(3),
      Token::CloseParen,
      Token::Plus,
      Token::OpenParen,
      Token::Number(4),
      Token::Times,
      Token::OpenParen,
      Token::Number(5),
      Token::Plus,
      Token::Number(6),
      Token::CloseParen,
      Token::CloseParen,
    ]);
    assert_eq!(
      infix_to_postfix(&data),
      VecDeque::from(vec![
        Token::Number(1),
        Token::Number(2),
        Token::Number(3),
        Token::Times,
        Token::Plus,
        Token::Number(4),
        Token::Number(5),
        Token::Number(6),
        Token::Plus,
        Token::Times,
        Token::Plus,
      ])
    );
  }
}
