use common::{Day, Part};
use itertools::Itertools;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-01-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(719_796, day.part_1.result);
    assert_eq!(144_554_112, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-01-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let parsed_data: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();

  let mut pair: Vec<u32> = Vec::new();

  for combo in parsed_data.iter().combinations(2) {
    if combo.iter().copied().sum1::<u32>().unwrap() == 2020 {
      pair = combo.iter().copied().copied().collect();
      break;
    }
  }

  // 462 * 1558 = 719_796
  (pair.iter().product1::<u32>().unwrap()) as u64
}

pub fn part_2(data: &[&str]) -> u64 {
  let parsed_data: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();

  let mut triple: Vec<u32> = Vec::new();

  for combo in parsed_data.iter().combinations(3) {
    if combo.iter().copied().sum1::<u32>().unwrap() == 2020 {
      triple = combo.iter().copied().copied().collect();
      break;
    }
  }

  // 979 * 366 * 675] = 144_554_112
  (triple.iter().product1::<u32>().unwrap()) as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    assert_eq!(
      part_1(&vec!["1721", "979", "366", "299", "675", "1456"]),
      514_579
    );
  }

  #[test]
  fn test_part_2() {
    assert_eq!(
      part_2(&vec!["1721", "979", "366", "299", "675", "1456"]),
      241_861_950
    );
  }
}
