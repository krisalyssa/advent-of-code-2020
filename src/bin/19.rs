use common::{Day, Part};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-19-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(198, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-19-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let (rule_lines, messages) = split_into_sections(&data);
  let rules = load_rules(&rule_lines);

  messages
    .iter()
    .map(|m| match_rule_0(&rules, m))
    .filter(|&m| m)
    .count() as u64
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

#[derive(Debug, PartialEq)]
enum Rule {
  A,
  B,
  And(Vec<u8>),
  Or(Vec<u8>, Vec<u8>),
}

#[derive(Debug, PartialEq)]
struct MatchResult<'a> {
  matching_part: &'a str,
  remainder: &'a str,
}

fn load_rules<'a>(data: &'a [&'a str]) -> HashMap<u8, Rule> {
  let re_terminal_a = Regex::new(r#"(\d+): "a""#).unwrap();
  let re_terminal_b = Regex::new(r#"(\d+): "b""#).unwrap();
  let re_and = Regex::new(r"(\d+): (\d+( \d+)*)").unwrap();
  let re_or = Regex::new(r"(\d+): (\d+( \d+)*) \| (\d+( \d+)*)").unwrap();

  let mut rules: HashMap<u8, Rule> = HashMap::new();

  for line in data {
    if re_terminal_a.is_match(line) {
      let captures = re_terminal_a.captures(line).unwrap();
      let rule_id = u8::from_str(captures.get(1).unwrap().as_str()).unwrap();
      rules.insert(rule_id, Rule::A);
    } else if re_terminal_b.is_match(line) {
      let captures = re_terminal_b.captures(line).unwrap();
      let rule_id = u8::from_str(captures.get(1).unwrap().as_str()).unwrap();
      rules.insert(rule_id, Rule::B);
    } else if re_or.is_match(line) {
      let captures = re_or.captures(line).unwrap();
      let rule_id = u8::from_str(captures.get(1).unwrap().as_str()).unwrap();
      let lhs = extract_dependent_ids(captures.get(2).unwrap().as_str());
      let rhs = extract_dependent_ids(captures.get(4).unwrap().as_str());
      rules.insert(rule_id, Rule::Or(lhs, rhs));
    } else if re_and.is_match(line) {
      let captures = re_and.captures(line).unwrap();
      let rule_id = u8::from_str(captures.get(1).unwrap().as_str()).unwrap();
      rules.insert(
        rule_id,
        Rule::And(extract_dependent_ids(captures.get(2).unwrap().as_str())),
      );
    } else {
      panic!("don't know how to parse ({})", line);
    }
  }

  rules
}

fn extract_dependent_ids(ids: &str) -> Vec<u8> {
  Regex::new(r"\s+")
    .unwrap()
    .split(ids)
    .map(|s| u8::from_str(s).unwrap())
    .collect()
}

fn match_rule_0(rules: &HashMap<u8, Rule>, text: &str) -> bool {
  if let Some(match_result) = match_rule_by_id(rules, 0, text) {
    match_result.remainder.is_empty()
  } else {
    false
  }
}

fn match_rule_by_id<'a>(
  rules: &HashMap<u8, Rule>,
  id: u8,
  text: &'a str,
) -> Option<MatchResult<'a>> {
  if let Some(rule) = rules.get(&id) {
    match_rule(rules, rule, text)
  } else {
    panic!("tried to get id = {} from rules = {:?}", id, rules);
  }
}

fn match_rule<'a>(
  rules: &HashMap<u8, Rule>,
  rule: &Rule,
  text: &'a str,
) -> Option<MatchResult<'a>> {
  match rule {
    Rule::A => {
      if let Some(remainder) = text.strip_prefix('a') {
        Some(MatchResult {
          matching_part: "a",
          remainder,
        })
      } else {
        None
      }
    }
    Rule::B => {
      if let Some(remainder) = text.strip_prefix('b') {
        Some(MatchResult {
          matching_part: "b",
          remainder,
        })
      } else {
        None
      }
    }
    Rule::And(subrule_ids) => {
      let mut remainder: &str = text;
      for sub_id in subrule_ids {
        if let Some(sub_match) = match_rule_by_id(rules, *sub_id, remainder) {
          remainder = sub_match.remainder;
        } else {
          return None;
        }
      }
      Some(MatchResult {
        matching_part: &text[0..(text.len() - remainder.len())],
        remainder,
      })
    }
    Rule::Or(lhs, rhs) => {
      if let Some(match_result) = match_rule(rules, &Rule::And(Vec::from(lhs.as_slice())), text) {
        Some(match_result)
      } else {
        match_rule(rules, &Rule::And(Vec::from(rhs.as_slice())), text)
      }
    }
  }
}

fn split_into_sections<'a>(data: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) {
  let vec: Vec<&[&str]> = data.split(|line| *line == "").collect();
  (vec[0], &vec[1])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "0: 4 1 5",
      "1: 2 3 | 3 2",
      "2: 4 4 | 5 5",
      "3: 4 5 | 5 4",
      "4: \"a\"",
      "5: \"b\"",
      "",
      "ababbb",
      "bababa",
      "abbbab",
      "aaabbb",
      "aaaabbb",
    ];
    assert_eq!(part_1(&data), 2);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_extract_dependent_ids() {
    assert_eq!(extract_dependent_ids("4 1 5"), vec![4, 1, 5]);
  }

  #[test]
  fn test_load_rules() {
    let data = vec![
      "0: 4 1 5",
      "1: 2 3 | 3 2",
      "2: 4 4 | 5 5",
      "3: 4 5 | 5 4",
      "4: \"a\"",
      "5: \"b\"",
    ];
    let rules = load_rules(&data);
    assert_eq!(rules.len(), 6);
    assert_eq!(*rules.get(&0).unwrap(), Rule::And(vec![4, 1, 5]));
    assert_eq!(*rules.get(&1).unwrap(), Rule::Or(vec![2, 3], vec![3, 2]));
    assert_eq!(*rules.get(&4).unwrap(), Rule::A);
    assert_eq!(*rules.get(&5).unwrap(), Rule::B);
  }

  #[test]
  fn test_match_rule_0() {
    let data = vec![
      "0: 4 1 5",
      "1: 2 3 | 3 2",
      "2: 4 4 | 5 5",
      "3: 4 5 | 5 4",
      "4: \"a\"",
      "5: \"b\"",
    ];
    let rules = load_rules(&data);
    assert_eq!(match_rule_0(&rules, "ababbb"), true);
    assert_eq!(match_rule_0(&rules, "bababa"), false);
    assert_eq!(match_rule_0(&rules, "abbbab"), true);
    assert_eq!(match_rule_0(&rules, "aaabbb"), false);
    assert_eq!(match_rule_0(&rules, "aaaabbb"), false);
  }

  #[test]
  fn test_match_rule_by_id() {
    let data = vec![
      "0: 4 1 5",
      "1: 2 3 | 3 2",
      "2: 4 4 | 5 5",
      "3: 4 5 | 5 4",
      "4: \"a\"",
      "5: \"b\"",
      "6: 4 5",
      "7: 4 5 4",
    ];
    let rules = load_rules(&data);
    assert_eq!(
      match_rule_by_id(&rules, 4, "a"),
      Some(MatchResult {
        matching_part: "a",
        remainder: ""
      })
    );
    assert_eq!(match_rule_by_id(&rules, 4, "b"), None);
    assert_eq!(match_rule_by_id(&rules, 6, "a"), None);
    assert_eq!(
      match_rule_by_id(&rules, 6, "ab"),
      Some(MatchResult {
        matching_part: "ab",
        remainder: ""
      })
    );
    assert_eq!(
      match_rule_by_id(&rules, 6, "aba"),
      Some(MatchResult {
        matching_part: "ab",
        remainder: "a"
      })
    );
    assert_eq!(
      match_rule_by_id(&rules, 7, "aba"),
      Some(MatchResult {
        matching_part: "aba",
        remainder: ""
      })
    );
    assert_eq!(
      match_rule_by_id(&rules, 3, "ab"),
      Some(MatchResult {
        matching_part: "ab",
        remainder: ""
      })
    );
    assert_eq!(
      match_rule_by_id(&rules, 3, "ba"),
      Some(MatchResult {
        matching_part: "ba",
        remainder: ""
      })
    );
    assert_eq!(match_rule_by_id(&rules, 3, "aab"), None);
  }

  #[test]
  fn test_split_into_sections() {
    let data = vec![
      "0: 4 1 5",
      "1: 2 3 | 3 2",
      "2: 4 4 | 5 5",
      "3: 4 5 | 5 4",
      "4: \"a\"",
      "5: \"b\"",
      "",
      "ababbb",
      "bababa",
      "abbbab",
      "aaabbb",
      "aaaabbb",
    ];

    let (rules, messages) = split_into_sections(&data);
    assert_eq!(rules.len(), 6);
    assert_eq!(messages.len(), 5);
  }
}
