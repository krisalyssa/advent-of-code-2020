use common::{Day, Part};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
  if let Ok(data) = common::load_data("data/day-02-input.txt") {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(655, day.part_1.result);
    assert_eq!(673, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-02-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &Vec<String>) -> u32 {
  data
    .iter()
    .filter(|rule| check_password_against_sled_rule(rule))
    .count() as u32
}

pub fn part_2(data: &Vec<String>) -> u32 {
  data
    .iter()
    .filter(|rule| check_password_against_toboggan_rule(rule))
    .count() as u32
}

fn check_password_against_sled_rule(rule: &str) -> bool {
  let regex = Regex::new(r"(\d+)-(\d+)\s+([a-z]):\s+(\S+)").unwrap();
  let captures = regex.captures(rule).unwrap();
  let password = captures.get(4).unwrap().as_str();
  let letter = captures.get(3).unwrap().as_str();
  let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
  let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

  valid_sled_password(password, letter, min, max)
}

fn valid_sled_password(password: &str, letter: &str, min: usize, max: usize) -> bool {
  let password_vec = password.graphemes(true).collect::<Vec<&str>>();
  let letter_count = password_vec.iter().filter(|c| **c == letter).count();

  letter_count >= min && letter_count <= max
}

fn check_password_against_toboggan_rule(rule: &str) -> bool {
  let regex = Regex::new(r"(\d+)-(\d+)\s+([a-z]):\s+(\S+)").unwrap();
  let captures = regex.captures(rule).unwrap();
  let password = captures.get(4).unwrap().as_str();
  let letter = captures.get(3).unwrap().as_str();
  let first = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
  let second = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

  valid_toboggan_password(password, letter, first, second)
}

fn valid_toboggan_password(password: &str, letter: &str, first: usize, second: usize) -> bool {
  let password_vec = password.graphemes(true).collect::<Vec<&str>>();
  (*password_vec.get(first - 1).unwrap() == letter)
    ^ (*password_vec.get(second - 1).unwrap() == letter)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_sled_password_1() {
    let password = "abcde";
    let letter = "a";
    let min = 1;
    let max = 3;

    assert_eq!(valid_sled_password(password, letter, min, max), true);
  }

  #[test]
  fn test_valid_sled_password_2() {
    let password = "cdefg";
    let letter = "b";
    let min = 1;
    let max = 3;

    assert_eq!(valid_sled_password(password, letter, min, max), false);
  }

  #[test]
  fn test_valid_sled_password_3() {
    let password = "ccccccccc";
    let letter = "c";
    let min = 2;
    let max = 9;

    assert_eq!(valid_sled_password(password, letter, min, max), true);
  }

  #[test]
  fn test_valid_toboggan_password_1() {
    let password = "abcde";
    let letter = "a";
    let first = 1;
    let second = 3;

    assert_eq!(
      valid_toboggan_password(password, letter, first, second),
      true
    );
  }

  #[test]
  fn test_valid_toboggan_password_2() {
    let password = "cdefg";
    let letter = "b";
    let first = 1;
    let second = 3;

    assert_eq!(
      valid_toboggan_password(password, letter, first, second),
      false
    );
  }

  #[test]
  fn test_valid_toboggan_password_3() {
    let password = "ccccccccc";
    let letter = "c";
    let first = 2;
    let second = 9;

    assert_eq!(
      valid_toboggan_password(password, letter, first, second),
      false
    );
  }
}
