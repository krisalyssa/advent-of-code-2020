use common::{Day, Part};

pub fn main() {
  if let Ok(data) = common::load_data("data/day01-input.txt") {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(3303995, day.part_1.result);
    assert_eq!(4953118, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day01-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(lines: &Vec<String>) -> u32 {
  let answer: u32 = lines
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .map(|value| fuel_for_mass(value))
    .sum();
  answer
}

pub fn part_2(lines: &Vec<String>) -> u32 {
  let answer: u32 = lines
    .iter()
    .filter_map(|value| value.parse::<u32>().ok())
    .map(|value| total_fuel_for_mass(value))
    .sum();
  answer
}

fn fuel_for_mass(mass: u32) -> u32 {
  if mass / 3 >= 2 {
    (mass / 3) - 2
  } else {
    0
  }
}

fn total_fuel_for_mass(mass: u32) -> u32 {
  let additional_fuel = fuel_for_mass(mass);
  if additional_fuel > 0 {
    additional_fuel + total_fuel_for_mass(additional_fuel)
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fuel_for_mass() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1969), 654);
    assert_eq!(fuel_for_mass(100756), 33583);
  }

  #[test]
  fn test_total_fuel_for_mass() {
    assert_eq!(total_fuel_for_mass(14), 2);
    assert_eq!(total_fuel_for_mass(1969), 966);
    assert_eq!(total_fuel_for_mass(100756), 50346);
  }
}
