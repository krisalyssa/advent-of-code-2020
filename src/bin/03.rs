use common::{Day, Part};
use std::collections::VecDeque;
use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
  if let Ok(data) = common::load_data("data/day-03-input.txt") {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(244, day.part_1.result);
    // assert_eq!(673, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-03-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &Vec<String>) -> u64 {
  let rows_vec = data
    .iter()
    .map(|line| split_row(line.as_str()))
    .map(|row| VecDeque::from(row))
    .collect::<Vec<VecDeque<&str>>>();
  let mut rows = VecDeque::from(rows_vec);

  let mut count = 0;

  while rows.len() > 0 {
    let mut row = rows.pop_front().unwrap();
    rows.iter_mut().for_each(|r| r.rotate_left(3));
    if row.pop_front().unwrap() == "#" {
      count = count + 1
    }
  }

  count
}

pub fn part_2(_data: &Vec<String>) -> u32 {
  0
}

fn split_row(row: &str) -> Vec<&str> {
  row.graphemes(true).collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "..##.......".to_string(),
      "#...#...#..".to_string(),
      ".#....#..#.".to_string(),
      "..#.#...#.#".to_string(),
      ".#...##..#.".to_string(),
      "..#.##.....".to_string(),
      ".#.#.#....#".to_string(),
      ".#........#".to_string(),
      "#.##...#...".to_string(),
      "#...##....#".to_string(),
      ".#..#...#.#".to_string(),
    ];

    assert_eq!(part_1(&data), 7);
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
