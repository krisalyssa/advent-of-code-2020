use common::{Day, Memory, Part};

pub fn main() {
  if let Ok(data) = common::load_data("data/day02-input.txt") {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    // assert_eq!(3303995, day.part_1.result);
    // assert_eq!(4953118, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day02-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(_data: &Vec<String>) -> u32 {
  // let answer: u32 = data
  //   .iter()
  //   .filter_map(|value| value.parse::<u32>().ok())
  //   .map(|value| fuel_for_mass(value))
  //   .sum();
  // answer
  0
}

pub fn part_2(_data: &Vec<String>) -> u32 {
  // let answer: u32 = data
  //   .iter()
  //   .filter_map(|value| value.parse::<u32>().ok())
  //   .map(|value| total_fuel_for_mass(value))
  //   .sum();
  // answer
  0
}

fn exec(memory: &mut Memory) {
  common::exec(memory)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let mut mem;

    mem = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    exec(&mut mem);
    assert_eq!(mem, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

    mem = vec![1, 0, 0, 0, 99];
    exec(&mut mem);
    assert_eq!(mem, vec![2, 0, 0, 0, 99]);

    mem = vec![2, 3, 0, 3, 99];
    exec(&mut mem);
    assert_eq!(mem, vec![2, 3, 0, 6, 99]);

    mem = vec![2, 4, 4, 5, 99, 0];
    exec(&mut mem);
    assert_eq!(mem, vec![2, 4, 4, 5, 99, 9801]);

    mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    exec(&mut mem);
    assert_eq!(mem, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }

  // #[test]
  // fn test_total_fuel_for_mass() {
  //   assert_eq!(total_fuel_for_mass(14), 2);
  //   assert_eq!(total_fuel_for_mass(1969), 966);
  //   assert_eq!(total_fuel_for_mass(100756), 50346);
  // }
}
