use common::{Day, Part};
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if let Ok(_) = common::load_data("data/day-04-input.txt", &mut data) {
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

pub fn part_1(data: &Vec<&str>) -> u64 {
  let mut buffer: Vec<String> = vec![];

  merge_records(&data, &mut buffer)
    .iter()
    .filter(|record| is_complete_record(record))
    .count() as u64
}

pub fn part_2(data: &Vec<&str>) -> u64 {
  let mut buffer: Vec<String> = vec![];

  merge_records(&data, &mut buffer)
    .iter()
    .filter(|record| is_complete_record(record))
    .filter(|record| is_valid_record(record))
    .count() as u64
}

fn is_complete_record(record: &HashMap<&str, &str>) -> bool {
  vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
    .iter()
    .all(|key| record.contains_key(key))
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

fn is_valid_record(record: &HashMap<&str, &str>) -> bool {
  vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    .iter()
    .all(|key| is_valid_field(*key, record.get(key).unwrap()))
}

fn merge_line_into_record(acc: &mut Vec<String>, line: &str) {
  if line.is_empty() {
    acc.push("".to_string());
  } else {
    if let Some(buffer) = acc.last_mut() {
      *buffer = [buffer, line].join(" ");
    }
  };
}

fn merge_records<'a>(
  data: &Vec<&str>,
  buffer: &'a mut Vec<String>,
) -> Vec<HashMap<&'a str, &'a str>> {
  if buffer.is_empty() {
    buffer.push("".to_string())
  };

  data
    .iter()
    .for_each(|line| merge_line_into_record(buffer, line));

  buffer
    .iter()
    .map(|record| record.trim())
    .map(|record| record_as_hash_map(&record))
    .collect()
}

fn record_as_hash_map(record: &str) -> HashMap<&str, &str> {
  let favs = (*record)
    .split_whitespace()
    .map(|fv| split_field_and_value(fv));
  let mut h = HashMap::new();
  for (field, value) in favs {
    h.insert(field, value);
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
  use maplit::hashmap;

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
    let record_1: HashMap<&str, &str> = hashmap! {
      "ecl" => "gry",
      "pid" => "860033327",
      "eyr" => "2020",
      "hcl" => "#fffffd",
      "byr" => "1937",
      "iyr" => "2017",
      "cid" => "147",
      "hgt" => "183cm",
    };
    assert!(is_complete_record(&record_1));

    let record_2: HashMap<&str, &str> = hashmap! {
      "iyr" => "2013",
      "ecl" => "amb",
      "cid" => "350",
      "eyr" => "2023",
      "pid" => "028048884",
      "hcl" => "#cfa07d",
      "byr" => "1929",
    };
    assert!(!is_complete_record(&record_2));

    let record_3: HashMap<&str, &str> = hashmap! {
      "hcl" => "#ae17e1",
      "iyr" => "2013",
      "eyr" => "2024",
      "ecl" => "brn",
      "pid" => "760753108",
      "byr" => "1931",
      "hgt" => "179cm",
    };
    assert!(is_complete_record(&record_3));

    let record_4: HashMap<&str, &str> = hashmap! {
      "hcl" => "#cfa07d",
      "eyr" => "2025",
      "pid" => "166559648",
      "iyr" => "2011",
      "ecl" => "brn",
      "hgt" => "59in",
    };
    assert!(!is_complete_record(&record_4));
  }

  #[test]
  fn test_is_valid_record() {
    let invalid_1: HashMap<&str, &str> = hashmap! {
      "eyr" => "1972",
      "cid" => "100",
      "hcl" => "#18171d",
      "ecl" => "amb",
      "hgt" => "170",
      "pid" => "186cm",
      "iyr" => "2018",
      "byr" => "1926",
    };
    assert!(!is_valid_record(&invalid_1));

    let valid_1: HashMap<&str, &str> = hashmap! {
      "pid" => "087499704",
      "hgt" => "74in",
      "ecl" => "grn",
      "iyr" => "2012",
      "eyr" => "2030",
      "byr" => "1980",
      "hcl" => "#623a2f",
    };
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
    let data: Vec<&str> = vec![
      "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
      "byr:1937 iyr:2017 cid:147 hgt:183cm",
      "",
      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
      "hcl:#cfa07d byr:1929",
      "",
      "hcl:#ae17e1 iyr:2013",
      "eyr:2024",
      "ecl:brn pid:760753108 byr:1931",
      "hgt:179cm",
      "",
      "hcl:#cfa07d eyr:2025 pid:166559648",
      "iyr:2011 ecl:brn hgt:59in",
    ];
    let mut buffer: Vec<String> = vec![];

    let actual = merge_records(&data, &mut buffer);
    let expected = hashmap! {
      "ecl" => "gry",
      "pid" => "860033327",
      "eyr" => "2020",
      "hcl" => "#fffffd",
      "byr" => "1937",
      "iyr" => "2017",
      "cid" => "147",
      "hgt" => "183cm",
    };

    assert_eq!(actual.len(), 4);
    assert_eq!(actual[0], expected);
  }
}
