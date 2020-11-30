use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::time::{Duration, Instant};

//*********************************************************
// IntCode interpreter
//

type Word = u32;

const ADD: Word = 1;
const MUL: Word = 2;
const HALT: Word = 99;

pub type Memory = Vec<u32>;

pub fn exec(memory: &mut Memory) {
  let mut pc: usize = 0;

  loop {
    let opcode = memory[pc];
    match opcode {
      ADD => {
        let addr_a = read(memory, pc + 1);
        let addr_b = read(memory, pc + 2);
        let addr_c = read(memory, pc + 3);

        let value_a = read(memory, addr_a as usize);
        let value_b = read(memory, addr_b as usize);

        println!(
          "ADD  [{}],[{}],[{}] # {} + {}",
          addr_a, addr_b, addr_c, value_a, value_b
        );
        write(memory, addr_c as usize, value_a + value_b);
        pc = pc + 4;
      }
      MUL => {
        let addr_a = read(memory, pc + 1);
        let addr_b = read(memory, pc + 2);
        let addr_c = read(memory, pc + 3);

        let value_a = read(memory, addr_a as usize);
        let value_b = read(memory, addr_b as usize);

        println!(
          "MUL  [{}],[{}],[{}] # {} * {}",
          addr_a, addr_b, addr_c, value_a, value_b
        );
        write(memory, addr_c as usize, value_a * value_b);
        pc = pc + 4;
      }
      HALT => {
        println!("HALT");
        break;
      }
      _ => panic!("invalid opcode {}", opcode),
    }
  }
}

fn read(memory: &Memory, address: usize) -> Word {
  memory[address]
}

fn write(memory: &mut Memory, address: usize, value: Word) {
  memory[address] = value;
}

//*********************************************************
// related to running days
//

pub type PartFn = fn(&Vec<String>) -> u32;

pub struct Part {
  fun: PartFn,
  pub result: u32,
  duration: Duration,
}

impl Part {
  pub fn new(fun: PartFn) -> Part {
    Part {
      fun,
      result: 0,
      duration: Duration::new(0, 0),
    }
  }
}

pub struct Day {
  pub part_1: Part,
  pub part_2: Part,
  duration: Duration,
}

impl Day {
  pub fn new(part_1: Part, part_2: Part) -> Day {
    Day {
      part_1,
      part_2,
      duration: Duration::new(0, 0),
    }
  }

  pub fn run(&mut self, data: &Vec<String>) {
    let start_part_1 = Instant::now();
    self.part_1.result = (self.part_1.fun)(data);
    self.part_1.duration = start_part_1.elapsed();
    let start_part_2 = Instant::now();
    self.part_2.result = (self.part_2.fun)(data);
    self.part_2.duration = start_part_2.elapsed();
    self.duration = start_part_1.elapsed();
  }

  pub fn to_string(&self) -> String {
    format!(
      "Time: part 1 = {} µs, part 2 = {} µs, total = {} µs",
      self.part_1.duration.as_micros(),
      self.part_2.duration.as_micros(),
      self.duration.as_micros()
    )
  }
}

pub fn load_csv(filename: &str) -> io::Result<Vec<String>> {
  let f = File::open(filename)?;
  load_csv_from_reader(f)
}

pub fn load_csv_from_reader<R: Read>(raw_reader: R) -> io::Result<Vec<String>> {
  let reader = BufReader::new(raw_reader);
  let v: Vec<String> = reader
    .lines()
    .map(|value| match value {
      Ok(v) => v,
      _ => "".to_string(),
    })
    .map(|line| {
      let re = Regex::new(r"\s*,\s*").unwrap();
      let fields: Vec<String> = re.split(&line).map(|field| String::from(field)).collect();
      fields
    })
    .flatten()
    .collect();

  Ok(v)
}

pub fn load_data(filename: &str) -> io::Result<Vec<String>> {
  let f = File::open(filename)?;
  load_data_from_reader(f)
}

fn load_data_from_reader<R: Read>(raw_reader: R) -> io::Result<Vec<String>> {
  let reader = BufReader::new(raw_reader);
  let iter = reader.lines().map(|value| match value {
    Ok(v) => v,
    _ => "".to_string(),
  });
  let v = Vec::from_iter(iter);
  Ok(v)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_csv_from_reader() {
    let data = "\
one,two,three
";
    match load_csv_from_reader(data.as_bytes()) {
      Ok(v) => assert_eq!(v, vec!["one", "two", "three"]),
      Err(err) => panic!("returned error: {}", err),
    }
  }

  #[test]
  fn test_load_multiline_csv_from_reader() {
    let data = "\
one,two,three
four,five,six
";
    match load_csv_from_reader(data.as_bytes()) {
      Ok(v) => assert_eq!(v, vec!["one", "two", "three", "four", "five", "six"]),
      Err(err) => panic!("returned error: {}", err),
    }
  }

  #[test]
  fn test_load_data_from_reader() {
    let data = String::from("one\ntwo\nthree");
    match load_data_from_reader(data.as_bytes()) {
      Ok(v) => assert_eq!(v, vec!["one", "two", "three"]),
      _ => panic!("returned error"),
    }
  }

  // #[test]
  // fn test_total_fuel_for_mass() {
  //   assert_eq!(total_fuel_for_mass(14), 2);
  //   assert_eq!(total_fuel_for_mass(1969), 966);
  //   assert_eq!(total_fuel_for_mass(100756), 50346);
  // }
}
