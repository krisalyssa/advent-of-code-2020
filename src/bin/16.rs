use common::{Day, Part};
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-16-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(26980, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-16-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let (rules_data, _my_ticket_data, nearby_tickets_data) = split_into_sections(&data);

  let rules: HashMap<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> = parse_rules(rules_data);

  // let my_ticket = parse_ticket(my_ticket_data[0]);
  let nearby_numbers: Vec<u32> = nearby_tickets_data
    .iter()
    .map(|line| parse_ticket(line))
    .flatten()
    .fold(Vec::new(), |mut acc, v| {
      acc.push(v);
      acc
    });

  let valid_ranges: Vec<RangeInclusive<u32>> = rules
    .values()
    .map(|(r1, r2)| vec![r1.clone(), r2.clone()])
    .flatten()
    .collect();

  let mut invalid_values: Vec<u32> = vec![];
  for value in &nearby_numbers {
    if !valid_ranges.iter().any(|r| r.contains(value)) {
      invalid_values.push(*value);
    }
  }

  invalid_values.iter().sum::<u32>() as u64
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn parse_rules<'a>(
  data: &'a [&str],
) -> HashMap<&'a str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
  let regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
  let mut vec: Vec<(&str, RangeInclusive<u32>, RangeInclusive<u32>)> = vec![];

  for line in data {
    let captures = regex.captures(line).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let range_1_start = u32::from_str(captures.get(2).unwrap().as_str()).unwrap();
    let range_1_end = u32::from_str(captures.get(3).unwrap().as_str()).unwrap();
    let range_2_start = u32::from_str(captures.get(4).unwrap().as_str()).unwrap();
    let range_2_end = u32::from_str(captures.get(5).unwrap().as_str()).unwrap();
    vec.push((
      name,
      RangeInclusive::new(range_1_start, range_1_end),
      RangeInclusive::new(range_2_start, range_2_end),
    ));
  }

  vec
    .iter()
    .fold(HashMap::new(), |mut acc, (name, range_1, range_2)| {
      acc.insert(name, (range_1.clone(), range_2.clone()));
      acc
    })
}

fn parse_ticket(line: &str) -> Vec<u32> {
  line.split(',').map(|v| u32::from_str(v).unwrap()).collect()
}

fn split_into_sections<'a>(data: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str], &'a [&'a str]) {
  let vec: Vec<&[&str]> = data.split(|line| *line == "").collect();
  (vec[0], &vec[1][1..], &vec[2][1..])
}

#[cfg(test)]
mod tests {
  use super::*;
  use maplit::hashmap;

  #[test]
  fn test_part_1() {
    let data = [
      "class: 1-3 or 5-7",
      "row: 6-11 or 33-44",
      "seat: 13-40 or 45-50",
      "",
      "your ticket:",
      "7,1,14",
      "",
      "nearby tickets:",
      "7,3,47",
      "40,4,50",
      "55,2,20",
      "38,6,12",
    ];
    assert_eq!(part_1(&data), 71);
  }

  #[test]
  fn test_part_2() {
    assert_eq!(part_2(&vec![]), 0);
  }

  #[test]
  fn test_parse_rules() {
    assert_eq!(
      parse_rules(&[
        "class: 1-3 or 5-7",
        "row: 6-11 or 33-44",
        "seat: 13-40 or 45-50"
      ]),
      hashmap! {
        "class" => ((1..=3), 5..=7),
        "row" => ((6..=11), (33..=44)),
        "seat" => ((13..=40), (45..=50)),
      }
    );
  }

  #[test]
  fn test_parse_ticket() {
    assert_eq!(parse_ticket("7,1,14"), vec![7, 1, 14]);
    assert_eq!(parse_ticket("7,3,47"), vec![7, 3, 47]);
    assert_eq!(parse_ticket("40,4,50"), vec![40, 4, 50]);
    assert_eq!(parse_ticket("55,2,20"), vec![55, 2, 20]);
    assert_eq!(parse_ticket("38,6,12"), vec![38, 6, 12]);
  }

  #[test]
  fn test_split_into_sections() {
    let data = [
      "class: 1-3 or 5-7",
      "row: 6-11 or 33-44",
      "seat: 13-40 or 45-50",
      "",
      "your ticket:",
      "7,1,14",
      "",
      "nearby tickets:",
      "7,3,47",
      "40,4,50",
      "55,2,20",
      "38,6,12",
    ];
    let (rules, my_ticket, nearby_tickets) = split_into_sections(&data);
    assert_eq!(rules.first().cloned().unwrap(), "class: 1-3 or 5-7");
    assert_eq!(my_ticket.first().cloned().unwrap(), "7,1,14");
    assert_eq!(nearby_tickets.first().cloned().unwrap(), "7,3,47");
  }
}
