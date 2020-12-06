use common::{Day, Part};
use regex::Regex;
use std::ops::Range;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-05-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(930, day.part_1.result);
    // assert_eq!(167, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-05-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  data
    .iter()
    .map(|pass| row_column_from_boarding_pass(pass))
    .map(|(row, column)| id_from_row_column(row, column))
    .max()
    .unwrap()
    .into()
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn binary_search(r: &Range<u32>, sequence: &str, bottom: char, top: char) -> u32 {
  let midpoint = midpoint_of_range(r);
  let c = sequence.chars().next().unwrap();

  if sequence.len() == 1 {
    match c {
      _ if c == bottom => r.start,
      _ if c == top => r.end,
      _ => panic!("{} not one of {},{}", c, bottom, top),
    }
  } else {
    match c {
      _ if c == bottom => binary_search(&(r.start..midpoint - 1), &sequence[1..], bottom, top),
      _ if c == top => binary_search(&(midpoint..r.end), &sequence[1..], bottom, top),
      _ => panic!("{} not one of {},{}", c, bottom, top),
    }
  }
}

fn column_from_boarding_pass(part: &str) -> u32 {
  binary_search(&(0..7), part, 'L', 'R')
}

fn id_from_row_column(row: u32, col: u32) -> u32 {
  (row * 8) + col
}

fn midpoint_of_range(r: &Range<u32>) -> u32 {
  (r.start + r.end + 1) / 2
}

fn row_from_boarding_pass(part: &str) -> u32 {
  binary_search(&(0..127), part, 'F', 'B')
}

fn row_column_from_boarding_pass(pass: &str) -> (u32, u32) {
  let regex = Regex::new(r"^([BF]{7})([LR]{3})$").unwrap();
  let captures = regex.captures(pass).unwrap();

  (
    row_from_boarding_pass(captures.get(1).unwrap().as_str()),
    column_from_boarding_pass(captures.get(2).unwrap().as_str()),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

    assert_eq!(part_1(&data), 820);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];

    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_binary_search() {
    assert_eq!(binary_search(&(0..127), "FBFBBFF", 'F', 'B'), 44);
    assert_eq!(binary_search(&(0..7), "RLR", 'L', 'R'), 5);
  }

  #[test]
  fn test_column_from_boarding_pass() {
    assert_eq!(column_from_boarding_pass("RRR"), 7);
    assert_eq!(column_from_boarding_pass("RRR"), 7);
    assert_eq!(column_from_boarding_pass("RLL"), 4);
  }

  #[test]
  fn test_id_from_row_column() {
    assert_eq!(id_from_row_column(70, 7), 567);
    assert_eq!(id_from_row_column(14, 7), 119);
    assert_eq!(id_from_row_column(102, 4), 820);
  }

  #[test]
  fn test_midpoint_of_range() {
    assert_eq!(midpoint_of_range(&(0..127)), 64);
    assert_eq!(midpoint_of_range(&(32..63)), 48);
    assert_eq!(midpoint_of_range(&(44..47)), 46);
  }

  #[test]
  fn test_row_from_boarding_pass() {
    assert_eq!(row_from_boarding_pass("BFFFBBF"), 70);
    assert_eq!(row_from_boarding_pass("FFFBBBF"), 14);
    assert_eq!(row_from_boarding_pass("BBFFBBF"), 102);
  }

  #[test]
  fn test_row_column_from_boarding_pass() {
    assert_eq!(row_column_from_boarding_pass("BFFFBBFRRR"), (70, 7));
    assert_eq!(row_column_from_boarding_pass("FFFBBBFRRR"), (14, 7));
    assert_eq!(row_column_from_boarding_pass("BBFFBBFRLL"), (102, 4));
  }
}
