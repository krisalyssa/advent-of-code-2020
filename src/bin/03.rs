use common::{Day, Part};
use std::collections::VecDeque;
use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-03-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(244, day.part_1.result);
    assert_eq!(9406609920, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-03-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  trees_in_path(data, 3, 1) as u64
}

pub fn part_2(data: &[&str]) -> u64 {
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|(right, down)| trees_in_path(data, *right as usize, *down as usize) as u64)
    .product()
}

fn split_row(row: &str) -> Vec<&str> {
  row.graphemes(true).collect::<Vec<&str>>()
}

// trees_in_path* should be general, but my first attempt wasn't popping
// the correct number of rows when down > 1, so I hard-coded it for now.
// I should revisit to allow passing an arbitrary value for down.

fn trees_in_path(data: &[&str], right: usize, down: usize) -> u32 {
  match down {
    1 => trees_in_path_down_1(data, right),
    2 => trees_in_path_down_2(data, right),
    _ => panic!("unsupported value for down"),
  }
}

fn trees_in_path_down_1(data: &[&str], right: usize) -> u32 {
  let rows_vec = data
    .iter()
    .map(|line| split_row(line))
    .map(VecDeque::from)
    .collect::<Vec<VecDeque<&str>>>();
  let mut rows = VecDeque::from(rows_vec);

  let mut count = 0;

  while !rows.is_empty() {
    let mut head = rows.pop_front().unwrap();
    if head.pop_front().unwrap() == "#" {
      count += 1
    }

    rows.iter_mut().for_each(|r| r.rotate_left(right));
  }

  count
}

fn trees_in_path_down_2(data: &[&str], right: usize) -> u32 {
  let rows_vec = data
    .iter()
    .map(|line| split_row(line))
    .map(VecDeque::from)
    .collect::<Vec<VecDeque<&str>>>();
  let mut rows = VecDeque::from(rows_vec);

  let mut count = 0;

  while !rows.is_empty() {
    let mut head = rows.pop_front().unwrap();
    if head.pop_front().unwrap() == "#" {
      count += 1
    }

    rows.pop_front();

    rows.iter_mut().for_each(|r| r.rotate_left(right));
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "..##.......",
      "#...#...#..",
      ".#....#..#.",
      "..#.#...#.#",
      ".#...##..#.",
      "..#.##.....",
      ".#.#.#....#",
      ".#........#",
      "#.##...#...",
      "#...##....#",
      ".#..#...#.#",
    ];

    assert_eq!(part_1(&data), 7);
  }

  #[test]
  fn test_part_2() {
    let data = vec![
      "..##.......",
      "#...#...#..",
      ".#....#..#.",
      "..#.#...#.#",
      ".#...##..#.",
      "..#.##.....",
      ".#.#.#....#",
      ".#........#",
      "#.##...#...",
      "#...##....#",
      ".#..#...#.#",
    ];

    assert_eq!(part_2(&data), 336);
  }

  #[test]
  fn test_trees_in_path() {
    let data = vec![
      "..##.......",
      "#...#...#..",
      ".#....#..#.",
      "..#.#...#.#",
      ".#...##..#.",
      "..#.##.....",
      ".#.#.#....#",
      ".#........#",
      "#.##...#...",
      "#...##....#",
      ".#..#...#.#",
    ];

    assert_eq!(trees_in_path(&data, 1, 1), 2);
    assert_eq!(trees_in_path(&data, 3, 1), 7);
    assert_eq!(trees_in_path(&data, 5, 1), 3);
    assert_eq!(trees_in_path(&data, 7, 1), 4);
    assert_eq!(trees_in_path(&data, 1, 2), 2);
  }

  #[test]
  fn test_split_row() {
    assert_eq!(
      split_row("..##......."),
      vec![".", ".", "#", "#", ".", ".", ".", ".", ".", ".", "."]
    );

    assert_eq!(
      split_row("#...#...#.."),
      vec!["#", ".", ".", ".", "#", ".", ".", ".", "#", ".", "."]
    );

    assert_eq!(
      split_row(".#....#..#."),
      vec![".", "#", ".", ".", ".", ".", "#", ".", ".", "#", "."]
    );
  }
}
