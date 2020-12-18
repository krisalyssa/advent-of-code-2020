use common::{Day, Part};
use lexer::*;
use std::collections::VecDeque;
use std::fmt;
use std::iter::FromIterator;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-18-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(1451467526514, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-18-input.txt");
    std::process::exit(1);
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum TokenValue {
  Number(u64),
  Plus,
  Times,
  OpenParen,
  CloseParen,
}

impl fmt::Display for TokenValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TokenValue::Number(n) => write!(f, "{}", *n),
      TokenValue::Plus => write!(f, "+"),
      TokenValue::Times => write!(f, "*"),
      TokenValue::OpenParen => write!(f, "("),
      TokenValue::CloseParen => write!(f, ")"),
    }
  }
}

type Token = lexer::Token<TokenValue>;
type TokenError = lexer::TokenError<&'static str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<Token, TokenError> for WhitespaceReader {
  #[inline(always)]
  fn priority(&self) -> usize {
    0
  }

  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    _: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_whitespace() {
          while let Some(ch) = input.peek(next, 0) {
            if ch.is_whitespace() {
              input.read(next);
            } else {
              break;
            }
          }

          ReaderResult::Empty
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberReader;

impl Reader<Token, TokenError> for NumberReader {
  #[inline(always)]
  fn priority(&self) -> usize {
    1
  }

  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_numeric() {
          let mut string = String::new();

          string.push(ch);

          while let Some(ch) = input.peek(next, 0) {
            if ch.is_numeric() {
              input.read(next);
              string.push(ch);
            } else {
              break;
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::Number(string.parse().unwrap()),
          ))
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OpReader;

impl Reader<Token, TokenError> for OpReader {
  #[inline(always)]
  fn priority(&self) -> usize {
    2
  }

  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => match ch {
        '+' => ReaderResult::Some(Token::new(
          TokenMeta::new_state_meta(current, next),
          TokenValue::Plus,
        )),
        '*' => ReaderResult::Some(Token::new(
          TokenMeta::new_state_meta(current, next),
          TokenValue::Times,
        )),
        '(' => ReaderResult::Some(Token::new(
          TokenMeta::new_state_meta(current, next),
          TokenValue::OpenParen,
        )),
        ')' => ReaderResult::Some(Token::new(
          TokenMeta::new_state_meta(current, next),
          TokenValue::CloseParen,
        )),
        _ => ReaderResult::None,
      },
      None => ReaderResult::None,
    }
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  data
    .iter()
    .map(|exp| parse(exp))
    .map(|tokens| evaluate_infix(&tokens))
    .sum::<u64>() as u64
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn evaluate_infix(infix: &VecDeque<TokenValue>) -> u64 {
  evaluate_postfix(&infix_to_postfix(infix))
}

fn evaluate_postfix(postfix: &VecDeque<TokenValue>) -> u64 {
  let mut stack: VecDeque<u64> = VecDeque::new();

  for token in postfix.iter() {
    match token {
      TokenValue::Number(number) => stack.push_front(*number),
      TokenValue::Plus => {
        let x = stack.pop_front().unwrap();
        let y = stack.pop_front().unwrap();
        stack.push_front(x + y);
      }
      TokenValue::Times => {
        let x = stack.pop_front().unwrap();
        let y = stack.pop_front().unwrap();
        stack.push_front(x * y);
      }
      TokenValue::OpenParen => {}
      TokenValue::CloseParen => {}
    }
  }

  stack.pop_front().unwrap()
}

fn infix_to_postfix(infix: &VecDeque<TokenValue>) -> VecDeque<TokenValue> {
  let mut postfix: VecDeque<TokenValue> = VecDeque::with_capacity(infix.capacity());
  let mut stack: VecDeque<TokenValue> = VecDeque::new();

  for token in infix.iter() {
    match token {
      TokenValue::Number(number) => postfix.push_back(TokenValue::Number(*number)),
      TokenValue::OpenParen => stack.push_front(TokenValue::OpenParen),
      TokenValue::CloseParen => {
        while stack.front() != Some(&TokenValue::OpenParen) {
          let op = stack.pop_front().unwrap();
          postfix.push_back(op);
        }
        stack.pop_front(); // discard the open paren
      }
      TokenValue::Plus => {
        while !stack.is_empty() && stack.front() != Some(&TokenValue::OpenParen) {
          postfix.push_back(stack.pop_front().unwrap());
        }
        stack.push_front(TokenValue::Plus);
      }
      TokenValue::Times => {
        while !stack.is_empty() && stack.front() != Some(&TokenValue::OpenParen) {
          postfix.push_back(stack.pop_front().unwrap());
        }
        stack.push_front(TokenValue::Times);
      }
    }
  }

  // drain anything remaining on the stack
  while !stack.is_empty() {
    postfix.push_back(stack.pop_front().unwrap());
  }

  postfix
}

fn parse(expression: &str) -> VecDeque<TokenValue> {
  let readers = ReadersBuilder::new()
    .add(WhitespaceReader)
    .add(NumberReader)
    .add(OpReader)
    .build();
  let lexer = readers.lexer(expression.chars());
  let tokens: Vec<Token> = lexer.map(Result::unwrap).collect();
  let token_values: Vec<TokenValue> = tokens.iter().map(lexer::Token::value).cloned().collect();
  VecDeque::from_iter(token_values.into_iter())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "1 + 2 * 3 + 4 * 5 + 6",
      "1 + (2 * 3) + (4 * (5 + 6))",
      "2 * 3 + (4 * 5)",
      "5 + (8 * 3 + 9 + 3 * 4 * 3)",
      "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
      "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    ];
    assert_eq!(part_1(&data), 26457);
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
        TokenValue::Number(1),
        TokenValue::Plus,
        TokenValue::Number(2),
        TokenValue::Times,
        TokenValue::Number(3),
        TokenValue::Plus,
        TokenValue::Number(4),
        TokenValue::Times,
        TokenValue::Number(5),
        TokenValue::Plus,
        TokenValue::Number(6),
      ])),
      71
    );
    assert_eq!(
      evaluate_infix(&VecDeque::from(vec![
        TokenValue::Number(1),
        TokenValue::Plus,
        TokenValue::OpenParen,
        TokenValue::Number(2),
        TokenValue::Times,
        TokenValue::Number(3),
        TokenValue::CloseParen,
        TokenValue::Plus,
        TokenValue::OpenParen,
        TokenValue::Number(4),
        TokenValue::Times,
        TokenValue::OpenParen,
        TokenValue::Number(5),
        TokenValue::Plus,
        TokenValue::Number(6),
        TokenValue::CloseParen,
        TokenValue::CloseParen,
      ])),
      51
    );
  }
  #[test]
  fn test_infix_to_postfix_simple() {
    let data: VecDeque<TokenValue> = VecDeque::from(vec![
      TokenValue::Number(1),
      TokenValue::Plus,
      TokenValue::Number(2),
      TokenValue::Times,
      TokenValue::Number(3),
      TokenValue::Plus,
      TokenValue::Number(4),
      TokenValue::Times,
      TokenValue::Number(5),
      TokenValue::Plus,
      TokenValue::Number(6),
    ]);
    assert_eq!(
      infix_to_postfix(&data),
      VecDeque::from(vec![
        TokenValue::Number(1),
        TokenValue::Number(2),
        TokenValue::Plus,
        TokenValue::Number(3),
        TokenValue::Times,
        TokenValue::Number(4),
        TokenValue::Plus,
        TokenValue::Number(5),
        TokenValue::Times,
        TokenValue::Number(6),
        TokenValue::Plus,
      ])
    );
  }

  #[test]
  fn test_infix_to_postfix_parens() {
    let data: VecDeque<TokenValue> = VecDeque::from(vec![
      TokenValue::Number(1),
      TokenValue::Plus,
      TokenValue::OpenParen,
      TokenValue::Number(2),
      TokenValue::Times,
      TokenValue::Number(3),
      TokenValue::CloseParen,
      TokenValue::Plus,
      TokenValue::OpenParen,
      TokenValue::Number(4),
      TokenValue::Times,
      TokenValue::OpenParen,
      TokenValue::Number(5),
      TokenValue::Plus,
      TokenValue::Number(6),
      TokenValue::CloseParen,
      TokenValue::CloseParen,
    ]);
    assert_eq!(
      infix_to_postfix(&data),
      VecDeque::from(vec![
        TokenValue::Number(1),
        TokenValue::Number(2),
        TokenValue::Number(3),
        TokenValue::Times,
        TokenValue::Plus,
        TokenValue::Number(4),
        TokenValue::Number(5),
        TokenValue::Number(6),
        TokenValue::Plus,
        TokenValue::Times,
        TokenValue::Plus,
      ])
    );
  }

  #[test]
  fn test_parse() {
    let expression = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(
      parse(expression),
      VecDeque::from(vec![
        TokenValue::Number(1),
        TokenValue::Plus,
        TokenValue::OpenParen,
        TokenValue::Number(2),
        TokenValue::Times,
        TokenValue::Number(3),
        TokenValue::CloseParen,
        TokenValue::Plus,
        TokenValue::OpenParen,
        TokenValue::Number(4),
        TokenValue::Times,
        TokenValue::OpenParen,
        TokenValue::Number(5),
        TokenValue::Plus,
        TokenValue::Number(6),
        TokenValue::CloseParen,
        TokenValue::CloseParen,
      ])
    );
  }
}
