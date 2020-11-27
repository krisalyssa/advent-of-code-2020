use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::time::{Duration, Instant};

pub fn run() -> Option<(Duration, Duration, Duration)> {
  if let Ok(lines) = load_data("data/day01-input.txt") {
    let start_part_1 = Instant::now();
    assert_eq!(3303995, part_1(&lines));
    let time_part_1 = start_part_1.elapsed();
    let start_part_2 = Instant::now();
    assert_eq!(4953118, part_2(&lines));
    let time_part_2 = start_part_2.elapsed();
    let time_total = start_part_1.elapsed();
    Some((time_part_1, time_part_2, time_total))
  } else {
    None
  }
}

pub fn part_1(lines: &Vec<String>) -> u32 {
  let answer: u32 = lines
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .map(|value| fuel_for_mass(value))
    .sum();
  answer
}

pub fn part_2(lines: &Vec<String>) -> u32 {
  let answer: u32 = lines
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .map(|value| total_fuel_for_mass(value))
    .sum();
  answer
}

fn fuel_for_mass(mass: u32) -> u32 {
  if mass / 3 >= 2 {
    (mass / 3) - 2
  } else {
    0
  }
}

fn load_data(filename: &str) -> io::Result<Vec<String>> {
  let f = File::open(filename)?;
  let reader = BufReader::new(f);
  let iter = reader.lines().map(|value| match value {
    Ok(v) => v,
    _ => "".to_string(),
  });
  let v = Vec::from_iter(iter);
  Ok(v)
}

fn total_fuel_for_mass(mass: u32) -> u32 {
  let additional_fuel = fuel_for_mass(mass);
  if additional_fuel > 0 {
    additional_fuel + total_fuel_for_mass(additional_fuel)
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fuel_for_mass() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1969), 654);
    assert_eq!(fuel_for_mass(100756), 33583);
  }

  #[test]
  fn test_total_fuel_for_mass() {
    assert_eq!(total_fuel_for_mass(14), 2);
    assert_eq!(total_fuel_for_mass(1969), 966);
    assert_eq!(total_fuel_for_mass(100756), 50346);
  }
}
