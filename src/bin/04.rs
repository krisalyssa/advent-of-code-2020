use common::{Day, Part};
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
  if let Ok(data) = common::load_data("data/day-04-input.txt") {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(208, day.part_1.result);
    assert_eq!(167, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-04-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &Vec<String>) -> u64 {
  merge_records(data)
    .iter()
    .filter(|record| is_complete_record(record))
    .count() as u64
}

pub fn part_2(data: &Vec<String>) -> u64 {
  merge_records(data)
    .iter()
    .filter(|record| is_complete_record(record))
    .filter(|record| is_valid_record(record))
    .count() as u64
}

fn is_complete_record(record: &HashMap<String, String>) -> bool {
  vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
    .iter()
    .all(|key| record.contains_key(&key.to_string()))
}

fn is_valid_field(field: &str, value: &str) -> bool {
  match field {
    "byr" => is_valid_byr(value),
    "iyr" => is_valid_iyr(value),
    "eyr" => is_valid_eyr(value),
    "hgt" => is_valid_hgt(value),
    "hcl" => is_valid_hcl(value),
    "ecl" => is_valid_ecl(value),
    "pid" => is_valid_pid(value),
    _ => true,
  }
}

fn is_valid_byr(value: &str) -> bool {
  let regex = Regex::new(r"^\d{4}$").unwrap();
  if regex.is_match(value) {
    let year = value.parse::<u32>().unwrap();
    (year >= 1920) && (year <= 2002)
  } else {
    false
  }
}

fn is_valid_iyr(value: &str) -> bool {
  let regex = Regex::new(r"^\d{4}$").unwrap();
  if regex.is_match(value) {
    let year = value.parse::<u32>().unwrap();
    (year >= 2010) && (year <= 2020)
  } else {
    false
  }
}

fn is_valid_eyr(value: &str) -> bool {
  let regex = Regex::new(r"^\d{4}$").unwrap();
  if regex.is_match(value) {
    let year = value.parse::<u32>().unwrap();
    (year >= 2020) && (year <= 2030)
  } else {
    false
  }
}

fn is_valid_hgt(value: &str) -> bool {
  let regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
  if regex.is_match(value) {
    let captures = regex.captures(value).unwrap();
    let length = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    match captures.get(2).unwrap().as_str() {
      "cm" => (length >= 150) && (length <= 193),
      "in" => (length >= 59) && (length <= 76),
      _ => false,
    }
  } else {
    false
  }
}

fn is_valid_hcl(value: &str) -> bool {
  let regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  regex.is_match(value)
}

fn is_valid_ecl(value: &str) -> bool {
  let regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
  regex.is_match(value)
}

fn is_valid_pid(value: &str) -> bool {
  let regex = Regex::new(r"^\d{9}$").unwrap();
  regex.is_match(value)
}

fn is_valid_record(record: &HashMap<String, String>) -> bool {
  vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    .iter()
    .all(|key| is_valid_field(*key, record.get(&key.to_string()).unwrap().as_str()))
}

fn merge_line_into_record(acc: &mut Vec<String>, line: &String) -> Vec<String> {
  if line.is_empty() {
    acc.push("".to_string());
  } else {
    if let Some(buffer) = acc.last_mut() {
      *buffer = [buffer.as_str(), line.as_str()].join(" ");
    }
  }

  acc.to_vec()
}

fn merge_records(data: &Vec<String>) -> Vec<HashMap<String, String>> {
  data
    .iter()
    .fold(vec!["".to_string()], |mut acc, line| {
      merge_line_into_record(&mut acc, line)
    })
    .iter()
    .map(|record| record.trim().to_string())
    .map(|record| record_as_hash_map(&record))
    .collect()
}

fn record_as_hash_map(record: &String) -> HashMap<String, String> {
  let favs = (*record)
    .split_whitespace()
    .map(|fv| split_field_and_value(fv));
  let mut h = HashMap::new();
  for (field, value) in favs {
    h.insert(field.to_string(), value.to_string());
  }
  h
}

fn split_field_and_value(fav: &str) -> (&str, &str) {
  let parts: Vec<&str> = fav.split(':').collect();
  (parts[0], parts[1])
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
  fn test_is_complete_record() {
    let record_1: HashMap<String, String> = [
      ("ecl".to_string(), "gry".to_string()),
      ("pid".to_string(), "860033327".to_string()),
      ("eyr".to_string(), "2020".to_string()),
      ("hcl".to_string(), "#fffffd".to_string()),
      ("byr".to_string(), "1937".to_string()),
      ("iyr".to_string(), "2017".to_string()),
      ("cid".to_string(), "147".to_string()),
      ("hgt".to_string(), "183cm".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    assert!(is_complete_record(&record_1));

    let record_2: HashMap<String, String> = [
      ("iyr".to_string(), "2013".to_string()),
      ("ecl".to_string(), "amb".to_string()),
      ("cid".to_string(), "350".to_string()),
      ("eyr".to_string(), "2023".to_string()),
      ("pid".to_string(), "028048884".to_string()),
      ("hcl".to_string(), "#cfa07d".to_string()),
      ("byr".to_string(), "1929".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    assert!(!is_complete_record(&record_2));

    let record_3: HashMap<String, String> = [
      ("hcl".to_string(), "#ae17e1".to_string()),
      ("iyr".to_string(), "2013".to_string()),
      ("eyr".to_string(), "2024".to_string()),
      ("ecl".to_string(), "brn".to_string()),
      ("pid".to_string(), "760753108".to_string()),
      ("byr".to_string(), "1931".to_string()),
      ("hgt".to_string(), "179cm".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    assert!(is_complete_record(&record_3));

    let record_4: HashMap<String, String> = [
      ("hcl".to_string(), "#cfa07d".to_string()),
      ("eyr".to_string(), "2025".to_string()),
      ("pid".to_string(), "166559648".to_string()),
      ("iyr".to_string(), "2011".to_string()),
      ("ecl".to_string(), "brn".to_string()),
      ("hgt".to_string(), "59in".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    assert!(!is_complete_record(&record_4));
  }

  #[test]
  fn test_is_valid_record() {
    let invalid_1: HashMap<String, String> = [
      ("eyr".to_string(), "1972".to_string()),
      ("cid".to_string(), "100".to_string()),
      ("hcl".to_string(), "#18171d".to_string()),
      ("ecl".to_string(), "amb".to_string()),
      ("hgt".to_string(), "170".to_string()),
      ("pid".to_string(), "186cm".to_string()),
      ("iyr".to_string(), "2018".to_string()),
      ("byr".to_string(), "1926".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    assert!(!is_valid_record(&invalid_1));

    let valid_1: HashMap<String, String> = [
      ("pid".to_string(), "087499704".to_string()),
      ("hgt".to_string(), "74in".to_string()),
      ("ecl".to_string(), "grn".to_string()),
      ("iyr".to_string(), "2012".to_string()),
      ("eyr".to_string(), "2030".to_string()),
      ("byr".to_string(), "1980".to_string()),
      ("hcl".to_string(), "#623a2f".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    assert!(is_valid_record(&valid_1));
  }

  #[test]
  fn test_is_valid_byr() {
    assert!(is_valid_byr("2002"));
    assert!(!is_valid_byr("2003"));
  }

  #[test]
  fn test_is_valid_iyr() {
    assert!(is_valid_iyr("2020"));
    assert!(!is_valid_iyr("2021"));
  }

  #[test]
  fn test_is_valid_eyr() {
    assert!(is_valid_eyr("2030"));
    assert!(!is_valid_eyr("2031"));
  }

  #[test]
  fn test_is_valid_hgt() {
    assert!(is_valid_hgt("60in"));
    assert!(is_valid_hgt("190cm"));
    assert!(!is_valid_hgt("190in"));
    assert!(!is_valid_hgt("190"));
  }

  #[test]
  fn test_is_valid_hcl() {
    assert!(is_valid_hcl("#123abc"));
    assert!(!is_valid_hcl("#123abz"));
    assert!(!is_valid_hcl("123abc"));
  }

  #[test]
  fn test_is_valid_ecl() {
    assert!(is_valid_ecl("brn"));
    assert!(!is_valid_ecl("wat"));
  }

  #[test]
  fn test_is_valid_pid() {
    assert!(is_valid_pid("000000001"));
    assert!(!is_valid_pid("0123456789"));
  }

  #[test]
  fn test_merge_records() {
    let data = vec![
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
      "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
      "".to_string(),
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
      "hcl:#cfa07d byr:1929".to_string(),
      "".to_string(),
      "hcl:#ae17e1 iyr:2013".to_string(),
      "eyr:2024".to_string(),
      "ecl:brn pid:760753108 byr:1931".to_string(),
      "hgt:179cm".to_string(),
      "".to_string(),
      "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
      "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];
    let actual = merge_records(&data);
    let expected: HashMap<String, String> = [
      ("ecl".to_string(), "gry".to_string()),
      ("pid".to_string(), "860033327".to_string()),
      ("eyr".to_string(), "2020".to_string()),
      ("hcl".to_string(), "#fffffd".to_string()),
      ("byr".to_string(), "1937".to_string()),
      ("iyr".to_string(), "2017".to_string()),
      ("cid".to_string(), "147".to_string()),
      ("hgt".to_string(), "183cm".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(actual.len(), 4);
    assert_eq!(actual[0], expected);
  }
}
