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
    assert_eq!(2314037239808, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-10-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let adapters = adapters(&data);
  let count_1 = count_differences_of_span(&adapters, 1);
  let count_3 = count_differences_of_span(&adapters, 3);

  (count_1 * count_3) as u64
}

pub fn part_2(data: &[&str]) -> u64 {
  let adapters = adapters(&data);
  let differences = differences(&adapters);
  let indexes_of_threes = indexes_of_threes(&differences);
  let span_lengths_of_one = span_lengths_of_one(&indexes_of_threes);

  let max_span = *(span_lengths_of_one.iter().max().unwrap());
  let tribonacci = tribonacci(max_span as usize);

  span_lengths_of_one
    .iter()
    .map(|length| tribonacci[*length as usize] as u64)
    .product::<u64>()
}

fn adapters(data: &[&str]) -> Vec<u32> {
  let mut parsed_data: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();
  parsed_data.sort_unstable();
  parsed_data
}

fn chain_adapters(adapters: &[u32]) -> VecDeque<u32> {
  let mut chain: VecDeque<u32> = VecDeque::from_iter((*adapters).iter().copied());
  chain.push_front(0);
  let largest_adapter: u32 = *chain.back().unwrap();
  let device: u32 = largest_adapter + 3;
  chain.push_back(device);
  chain
}

fn count_differences_of_span(adapters: &[u32], span: usize) -> usize {
  differences(&adapters.to_vec())
    .iter()
    .filter(|d| **d == span as u32)
    .count()
}

fn differences(adapters: &[u32]) -> Vec<u32> {
  let chain = chain_adapters(adapters);

  Vec::from(chain)
    .windows(2)
    .map(|pair| pair[1] - pair[0])
    .collect()
}

fn indexes_of_threes(differences: &[u32]) -> Vec<u32> {
  differences
    .iter()
    .enumerate()
    .filter(|(_, diff)| **diff == 3)
    .map(|(ix, _)| (ix + 1) as u32)
    .collect()
}

fn span_lengths_of_one(indexes_of_threes: &[u32]) -> Vec<u32> {
  let mut v: Vec<u32> = Vec::from_iter((*indexes_of_threes).iter().copied());
  v.insert(0, 0);
  v.windows(2).map(|pair| pair[1] - pair[0] - 1).collect()
}

fn tribonacci(to_n: usize) -> Vec<u32> {
  let mut t: Vec<u32> = vec![1, 1, 2];
  while t.len() <= to_n {
    let ix_last = t.len();
    let ix_first = ix_last - 3;
    let last_3 = t.get(ix_first..ix_last).unwrap().to_vec();
    t.push(last_3.iter().sum());
  }
  t
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      assert_eq!(part_1(&data), 35);
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      assert_eq!(part_1(&data), 220);
    }
  }

  #[test]
  fn test_part_2() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      assert_eq!(part_2(&data), 8);
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      assert_eq!(part_2(&data), 19208);
    }
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
  fn test_count_differences_of_span() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      let adapters = adapters(&data);
      assert_eq!(count_differences_of_span(&adapters, 1), 7);
      assert_eq!(count_differences_of_span(&adapters, 3), 5);
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      let adapters = adapters(&data);
      assert_eq!(count_differences_of_span(&adapters, 1), 22);
      assert_eq!(count_differences_of_span(&adapters, 3), 10);
    }
  }

  #[test]
  fn test_differences() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      let adapters = adapters(&data);
      assert_eq!(
        differences(&adapters),
        vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3]
      );
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      let adapters = adapters(&data);
      assert_eq!(
        differences(&adapters),
        vec![
          1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 3, 1, 1, 1, 1, 3, 1, 3, 3, 1, 1, 1,
          1, 3
        ]
      );
    }
  }

  #[test]
  fn test_indexes_of_threes() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      let adapters = adapters(&data);
      let differences = differences(&adapters);
      assert_eq!(indexes_of_threes(&differences), vec![2, 6, 9, 11, 12]);
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      let adapters = adapters(&data);
      let differences = differences(&adapters);
      assert_eq!(
        indexes_of_threes(&differences),
        vec![5, 10, 11, 15, 18, 19, 24, 26, 27, 32]
      );
    }
  }

  #[test]
  fn test_span_lengths_of_one() {
    {
      let data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
      let adapters = adapters(&data);
      let differences = differences(&adapters);
      let indexes_of_threes = indexes_of_threes(&differences);
      assert_eq!(span_lengths_of_one(&indexes_of_threes), vec![1, 3, 2, 1, 0]);
    }
    {
      let data = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
      ];
      let adapters = adapters(&data);
      let differences = differences(&adapters);
      let indexes_of_threes = indexes_of_threes(&differences);
      assert_eq!(
        span_lengths_of_one(&indexes_of_threes),
        vec![4, 4, 0, 3, 2, 0, 4, 1, 0, 4]
      );
    }
  }

  #[test]
  fn test_tribonacci() {
    assert_eq!(tribonacci(3), vec![1, 1, 2, 4]);
    assert_eq!(tribonacci(5), vec![1, 1, 2, 4, 7, 13]);
    assert_eq!(
      tribonacci(10),
      vec![1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274]
    );
  }
}
