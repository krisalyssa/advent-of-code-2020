use common::{Day, Part};
use std::collections::VecDeque;
use std::iter::FromIterator;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-10-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(1856, day.part_1.result);
    // assert_eq!(93727241, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-10-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let adapters = adapters(&data);
  let (count_1, count_3) = count_differences(&adapters);
  (count_1 * count_3) as u64
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn adapters(data: &[&str]) -> Vec<u32> {
  let mut parsed_data: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();
  parsed_data.sort();
  parsed_data
}

fn count_differences(adapters: &Vec<u32>) -> (u32, u32) {
  let mut chain: VecDeque<u32> = VecDeque::from_iter((*adapters).iter().copied());
  chain.push_front(0);
  let largest_adapter: u32 = *chain.back().unwrap();
  let device: u32 = largest_adapter + 3;
  chain.push_back(device);

  let mut count_1 = 0;
  let mut count_3 = 0;

  for pair in Vec::from(chain).windows(2) {
    match pair[1] - pair[0] {
      1 => count_1 += 1,
      3 => count_3 += 1,
      _ => panic!("diff between {} and {} isn't 1 or 3", pair[1], pair[0]),
    }
  }

  (count_1, count_3)
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
  fn test_adapters() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      assert_eq!(
        adapters(&data),
        vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19,]
      );
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      assert_eq!(
        adapters(&data),
        vec![
          1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
          39, 42, 45, 46, 47, 48, 49,
        ]
      );
    }
  }

  #[test]
  fn test_count_differences() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      let adapters = adapters(&data);
      assert_eq!(count_differences(&adapters), (7, 5));
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      let adapters = adapters(&data);
      assert_eq!(count_differences(&adapters), (22, 10));
    }
  }
}
