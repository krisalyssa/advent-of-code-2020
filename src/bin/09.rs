use common::{Day, Part};
use itertools::Itertools;
use std::collections::VecDeque;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-09-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(756008079, day.part_1.result);
    assert_eq!(93727241, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-09-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  if let Some(retval) = find_first_step(data, 25) {
    retval as u64
  } else {
    0
  }
}

pub fn part_2(data: &[&str]) -> u64 {
  if let Some(retval) = find_weakness(data, 756008079) {
    retval as u64
  } else {
    0
  }
}

fn initialize(
  data: &[&str],
  preamble_size: usize,
  window: &mut VecDeque<u32>,
  remaining: &mut Vec<u32>,
) {
  let mut preamble: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();

  remaining.clear();
  let mut temp = preamble.split_off(preamble_size);
  remaining.append(&mut temp);

  window.clear();
  let mut preamble_as_vecdeque = VecDeque::from(preamble);
  window.append(&mut preamble_as_vecdeque);
}

fn find_first_step(data: &[&str], preamble_size: usize) -> Option<u32> {
  let mut window: VecDeque<u32> = VecDeque::with_capacity(preamble_size);
  let mut remaining: Vec<u32> = Vec::with_capacity(data.len() - preamble_size);
  initialize(&data, preamble_size, &mut window, &mut remaining);

  let mut retval = None;

  for num in remaining {
    if !window
      .iter()
      .combinations(2)
      .any(|combo| combo.iter().copied().sum1::<u32>().unwrap() == num)
    {
      retval = Some(num);
      break;
    }
    window.pop_front();
    window.push_back(num);
  }

  retval
}

fn find_weakness(data: &[&str], invalid_number: u32) -> Option<u32> {
  let dataset: Vec<u32> = data
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .collect();

  let mut found: Option<(usize, usize)> = None;

  for window_size in 2..dataset.len() {
    dataset.windows(window_size).enumerate().find(|(ix, w)| {
      if w.iter().sum::<u32>() == invalid_number {
        found = Some((*ix, window_size));
        true
      } else {
        false
      }
    });
  }

  if let Some((window_start, window_size)) = found {
    let min = dataset[window_start..window_start + window_size - 1]
      .iter()
      .min()
      .unwrap();
    let max = dataset[window_start..window_start + window_size - 1]
      .iter()
      .max()
      .unwrap();
    Some((min + max) as u32)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn test_find_first_step() {
    let data = vec![
      "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
      "127", "219", "299", "277", "309", "576",
    ];
    assert_eq!(find_first_step(&data, 5).unwrap(), 127);
  }

  #[test]
  fn test_find_weakness() {
    let data = vec![
      "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
      "127", "219", "299", "277", "309", "576",
    ];
    assert_eq!(find_weakness(&data, 127).unwrap(), 62);
  }

  #[test]
  fn test_initialize() {
    let data = vec![
      "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
      "127", "219", "299", "277", "309", "576",
    ];
    let preamble_size = 5;

    let mut window: VecDeque<u32> = VecDeque::with_capacity(preamble_size);
    let mut remaining: Vec<u32> = Vec::with_capacity(data.len());
    initialize(&data, preamble_size, &mut window, &mut remaining);

    println!("window = {:?}", window);
    assert_eq!(window.len(), preamble_size);
    assert_eq!(window.pop_front().unwrap(), u32::from_str(data[0]).unwrap());

    assert_eq!(remaining.len(), data.len() - preamble_size);
    assert_eq!(*remaining.get(0).unwrap(), u32::from_str(data[5]).unwrap());
  }
}
