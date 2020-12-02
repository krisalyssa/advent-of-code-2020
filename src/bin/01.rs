use common::{Day, Part};
use itertools::Itertools;
use std::collections::{BinaryHeap, VecDeque};

pub fn main() {
  if let Ok(data) = common::load_data("data/day-01-input.txt") {
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

pub fn part_1(data: &Vec<String>) -> u32 {
  let parsed_data: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();

  let sorted_data = sort_data(parsed_data);
  let mut deque: VecDeque<u32> = VecDeque::with_capacity(sorted_data.len());
  for item in sorted_data {
    deque.push_back(item);
  }

  let mut entry_1: Option<u32> = None;
  let mut entry_2: Option<u32> = None;

  while deque.len() > 1 {
    let left = *deque.front().unwrap();
    while left + *deque.back().unwrap() > 2020 {
      deque.pop_back();
    }
    let mut right_iter = deque.iter();
    right_iter.next();

    for right in right_iter {
      let sum = left + right;
      if sum == 2020 {
        entry_2 = Some(*right);
        break;
      }
    }

    if entry_2 != None {
      entry_1 = Some(left);
      break;
    }

    deque.pop_front();
  }

  // 462 * 1558 = 719_796
  entry_1.unwrap_or(0) * entry_2.unwrap_or(0)
}

pub fn part_2(data: &Vec<String>) -> u32 {
  let parsed_data: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();

  let mut triple: Vec<u32> = Vec::new();

  for combo in parsed_data.iter().combinations(3) {
    if combo.iter().map(|item| *item).sum1::<u32>().unwrap() == 2020 {
      triple = combo
        .iter()
        .map(|item| *item)
        .map(|item| *item)
        .collect_vec();
      break;
    }
  }

  // 979 * 366 * 675] = 144_554_112
  triple.iter().product1::<u32>().unwrap()
}

fn sort_data(data: Vec<u32>) -> Vec<u32> {
  let mut copied_data = Vec::new();
  copied_data.extend_from_slice(data.as_slice());
  BinaryHeap::from(copied_data).into_sorted_vec()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    assert_eq!(
      part_1(&vec![
        "1721".to_string(),
        "979".to_string(),
        "366".to_string(),
        "299".to_string(),
        "675".to_string(),
        "1456".to_string()
      ]),
      514_579
    );
  }

  #[test]
  fn test_part_2() {
    assert_eq!(
      part_2(&vec![
        "1721".to_string(),
        "979".to_string(),
        "366".to_string(),
        "299".to_string(),
        "675".to_string(),
        "1456".to_string()
      ]),
      241_861_950
    );
  }

  #[test]
  fn test_sort_data() {
    assert_eq!(
      sort_data(vec![1721, 979, 366, 299, 675, 1456]),
      vec![299, 366, 675, 979, 1456, 1721]
    );
  }
}
