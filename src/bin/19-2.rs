// many thanks for u/karjonas on Reddit, without whose solution I likely would still
// be at the bottom of a rabbit hole

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
    assert_eq!(372, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-19-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let (rule_lines, messages) = split_into_sections(&data);
  let rules = load_rules(&rule_lines);

  valid_messages(&rules, messages)
}

pub fn part_2(data: &[&str]) -> u64 {
  let (rule_lines, messages) = split_into_sections(&data);
  let mut rules = load_rules(&rule_lines);
  fix_rules(&mut rules);

  valid_messages(&rules, messages)
}

#[derive(Debug, PartialEq)]
enum Rule {
  A,
  B,
  And(Vec<u8>),
  Or(Vec<u8>, Vec<u8>),
}

fn load_rules<'a>(data: &'a [&'a str]) -> HashMap<u8, Rule> {
  let re_terminal_a = Regex::new(r#"(\d+): "a""#).unwrap();
  let re_terminal_b = Regex::new(r#"(\d+): "b""#).unwrap();
  let re_and = Regex::new(r"(\d+): (\d+( \d+)*)").unwrap();
  let re_or = Regex::new(r"(\d+): (\d+( \d+)*) \| (\d+( \d+)*)").unwrap();

  let mut rules: HashMap<u8, Rule> = HashMap::new();

  for line in data {
    // re_or *must* be checked before re_and!

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

fn fix_rules(rules: &mut HashMap<u8, Rule>) {
  rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
  rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));
}

fn get_rule_by_id(rules: &HashMap<u8, Rule>, id: u8) -> &Rule {
  if let Some(rule) = rules.get(&id) {
    rule
  } else {
    panic!("tried to get id = {} from rules = {:?}", id, rules);
  }
}

fn resolve(rules: &HashMap<u8, Rule>, rule_id: u8, text: &Vec<char>, ix: usize) -> Vec<usize> {
  if ix >= text.len() {
    return vec![];
  }

  let mut matches: Vec<usize> = vec![];

  match get_rule_by_id(rules, rule_id) {
    Rule::A => {
      if text[ix] == 'a' {
        return [ix + 1].to_vec();
      } else {
        return vec![];
      }
    }
    Rule::B => {
      if text[ix] == 'b' {
        return [ix + 1].to_vec();
      } else {
        return vec![];
      }
    }
    Rule::And(subrule_ids) => {
      matches = resolve_and(rules, subrule_ids, text, ix);
    }
    Rule::Or(lhs, rhs) => {
      matches.append(&mut resolve_and(rules, lhs, text, ix));
      matches.append(&mut resolve_and(rules, rhs, text, ix));
    }
  }

  matches
}

fn resolve_and(
  rules: &HashMap<u8, Rule>,
  rule_ids: &[u8],
  text: &Vec<char>,
  ix: usize,
) -> Vec<usize> {
  let mut ixs = [ix].to_vec();

  for rule_id in rule_ids {
    let mut new_ixs = vec![];
    for jx in &ixs {
      new_ixs.append(&mut resolve(rules, *rule_id, text, *jx));
    }

    ixs = new_ixs;

    if ixs.is_empty() {
      break;
    }
  }

  ixs
}

fn split_into_sections<'a>(data: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) {
  let vec: Vec<&[&str]> = data.split(|line| *line == "").collect();
  (vec[0], &vec[1])
}

fn valid_messages(rules: &HashMap<u8, Rule>, messages: &[&str]) -> u64 {
  let mut count = 0;
  for message in messages {
    if resolve(&rules, 0, &message.chars().collect(), 0)
      .iter()
      .any(|ix| *ix == message.len())
    {
      count += 1;
    }
  }

  count
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
    let data = vec![
      "0: 8 11",
      "1: \"a\"",
      "2: 1 24 | 14 4",
      "3: 5 14 | 16 1",
      "4: 1 1",
      "5: 1 14 | 15 1",
      "6: 14 14 | 1 14",
      "7: 14 5 | 1 21",
      "8: 42",
      "9: 14 27 | 1 26",
      "10: 23 14 | 28 1",
      "11: 42 31",
      "12: 24 14 | 19 1",
      "13: 14 3 | 1 12",
      "14: \"b\"",
      "15: 1 | 14",
      "16: 15 1 | 14 14",
      "17: 14 2 | 1 7",
      "18: 15 15",
      "19: 14 1 | 14 14",
      "20: 14 14 | 1 15",
      "21: 14 1 | 1 14",
      "22: 14 14",
      "23: 25 1 | 22 14",
      "24: 14 1",
      "25: 1 1 | 1 14",
      "26: 14 22 | 1 20",
      "27: 1 6 | 14 18",
      "28: 16 1",
      "31: 14 17 | 1 13",
      "42: 9 14 | 10 1",
      "",
      "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
      "bbabbbbaabaabba",
      "babbbbaabbbbbabbbbbbaabaaabaaa",
      "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
      "bbbbbbbaaaabbbbaaabbabaaa",
      "bbbababbbbaaaaaaaabbababaaababaabab",
      "ababaaaaaabaaab",
      "ababaaaaabbbaba",
      "baabbaaaabbaaaababbaababb",
      "abbbbabbbbaaaababbbbbbaaaababb",
      "aaaaabbaabaaaaababaa",
      "aaaabbaaaabbaaa",
      "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
      "babaaabbbaaabaababbaabababaaab",
      "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    ];
    assert_eq!(part_1(&data), 3);
    assert_eq!(part_2(&data), 12);
  }

  #[test]
  fn test_extract_dependent_ids() {
    assert_eq!(extract_dependent_ids("4 1 5"), vec![4, 1, 5]);
  }

  #[test]
  fn test_fix_rules() {
    let data = vec![
      "0: 4 1 5",
      "1: 2 3 | 3 2",
      "2: 4 4 | 5 5",
      "3: 4 5 | 5 4",
      "4: \"a\"",
      "5: \"b\"",
      "8: 42",
      "11: 42 31",
    ];
    let mut rules = load_rules(&data);
    assert_eq!(*rules.get(&8).unwrap(), Rule::And(vec![42]));
    assert_eq!(*rules.get(&11).unwrap(), Rule::And(vec![42, 31]));

    fix_rules(&mut rules);
    assert_eq!(*rules.get(&8).unwrap(), Rule::Or(vec![42], vec![42, 8]));
    assert_eq!(
      *rules.get(&11).unwrap(),
      Rule::Or(vec![42, 31], vec![42, 11, 31])
    );
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
