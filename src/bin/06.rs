use common::{Day, Part};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-06-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(6714, day.part_1.result);
    assert_eq!(3435, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-06-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut buffer: Vec<String> = vec![];

  merge_lines_into_record_strs(&data, &mut buffer)
    .iter()
    .map(|group| count_any_answers(group))
    .sum::<u64>()
}

pub fn part_2(data: &[&str]) -> u64 {
  let mut buffer: Vec<Vec<String>> = vec![];

  merge_lines_into_record_vecs(&data, &mut buffer)
    .iter()
    .map(|group| count_all_answers(group))
    .sum::<u64>()
}

fn count_all_answers(group: &[String]) -> u64 {
  let mut iter = group.iter();
  let mut first_group: HashSet<char> = HashSet::from_iter(iter.next().unwrap().chars());
  for next_str in iter {
    let next_group: HashSet<char> = HashSet::from_iter(next_str.chars());
    first_group = first_group.intersection(&next_group).copied().collect();
  }

  first_group.len() as u64
}

fn count_any_answers(group: &str) -> u64 {
  let mut answers: HashSet<char> = HashSet::new();
  for c in group.chars() {
    answers.insert(c);
  }
  answers.len() as u64
}

fn merge_line_into_str(acc: &mut Vec<String>, line: &str) {
  if line.is_empty() {
    acc.push("".to_string());
  } else if let Some(buffer) = acc.last_mut() {
    *buffer = [buffer, line].join("");
  };
}

fn merge_line_into_vec(acc: &mut Vec<Vec<String>>, line: &str) {
  if line.is_empty() {
    acc.push(vec![]);
  } else if let Some(buffer) = acc.last_mut() {
    buffer.push(line.to_string());
  };
}

fn merge_lines_into_record_strs(data: &[&str], buffer: &mut Vec<String>) -> Vec<String> {
  if buffer.is_empty() {
    buffer.push("".to_string())
  };

  data
    .iter()
    .map(|line| line.trim())
    .for_each(|line| merge_line_into_str(buffer, line));

  buffer.to_vec()
}

fn merge_lines_into_record_vecs(data: &[&str], buffer: &mut Vec<Vec<String>>) -> Vec<Vec<String>> {
  if buffer.is_empty() {
    buffer.push(vec![]);
  }

  data
    .iter()
    .map(|line| line.trim())
    .for_each(|line| merge_line_into_vec(buffer, line));

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
    let data = vec![
      "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];
    assert_eq!(part_2(&data), 6);
  }

  #[test]
  fn test_count_all_answers() {
    assert_eq!(count_all_answers(&vec!["abc".to_string()]), 3);
    assert_eq!(
      count_all_answers(&vec!["a".to_string(), "b".to_string(), "c".to_string()]),
      0
    );
    assert_eq!(
      count_all_answers(&vec!["ab".to_string(), "ac".to_string()]),
      1
    );
    assert_eq!(
      count_all_answers(&vec![
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string()
      ]),
      1
    );
    assert_eq!(count_all_answers(&vec!["b".to_string()]), 1);
  }

  #[test]
  fn test_count_any_answers() {
    assert_eq!(count_any_answers("abc"), 3);
    assert_eq!(count_any_answers("abac"), 3);
    assert_eq!(count_any_answers("aaaa"), 1);
    assert_eq!(count_any_answers("b"), 1);
  }

  #[test]
  fn test_merge_lines_into_record_strs() {
    let mut buffer: Vec<String> = vec![];
    assert_eq!(
      merge_lines_into_record_strs(
        &vec!["abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b"],
        &mut buffer
      ),
      vec!["abc", "abc", "abac", "aaaa", "b"]
    );
  }

  #[test]
  fn test_merge_lines_into_record_vecs() {
    let mut buffer: Vec<Vec<String>> = vec![];
    assert_eq!(
      merge_lines_into_record_vecs(
        &vec!["abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b"],
        &mut buffer
      ),
      vec![
        vec!["abc"],
        vec!["a", "b", "c"],
        vec!["ab", "ac"],
        vec!["a", "a", "a", "a"],
        vec!["b"]
      ]
    );
  }
}
