use common::{Day, Part};
use std::collections::HashSet;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-06-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(6714, day.part_1.result);
    assert_eq!(515, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-06-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut buffer: Vec<String> = vec![];

  merge_records(&data, &mut buffer)
    .iter()
    .map(|group| count_answers(group))
    .sum::<u64>()
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn count_answers(group: &str) -> u64 {
  let mut answers: HashSet<char> = HashSet::new();
  for c in group.chars() {
    answers.insert(c);
  }
  answers.len() as u64
}

fn merge_line_into_record(acc: &mut Vec<String>, line: &str) {
  if line.is_empty() {
    acc.push("".to_string());
  } else if let Some(buffer) = acc.last_mut() {
    *buffer = [buffer, line].join("");
  };
}

fn merge_records(data: &[&str], buffer: &mut Vec<String>) -> Vec<String> {
  if buffer.is_empty() {
    buffer.push("".to_string())
  };

  data
    .iter()
    .map(|line| line.trim())
    .for_each(|line| merge_line_into_record(buffer, line));

  buffer.to_vec()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];
    assert_eq!(part_1(&data), 11);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_1(&data), 0);
  }

  #[test]
  fn test_count_answers() {
    assert_eq!(count_answers("abc"), 3);
    assert_eq!(count_answers("abac"), 3);
    assert_eq!(count_answers("aaaa"), 1);
    assert_eq!(count_answers("b"), 1);
  }

  #[test]
  fn test_merge_records() {
    let mut buffer: Vec<String> = vec![];
    assert_eq!(
      merge_records(
        &vec!["abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b"],
        &mut buffer
      ),
      vec!["abc", "abc", "abac", "aaaa", "b"]
    );
  }
}
