use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::time::{Duration, Instant};

pub type PartFn = fn(&Vec<String>) -> u32;

pub struct Part {
  fun: PartFn,
  pub result: u32,
  duration: Duration,
}

pub struct Day {
  pub part_1: Part,
  pub part_2: Part,
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

pub fn load_data(filename: &str) -> io::Result<Vec<String>> {
  let f = File::open(filename)?;
  let reader = BufReader::new(f);
  let iter = reader.lines().map(|value| match value {
    Ok(v) => v,
    _ => "".to_string(),
  });
  let v = Vec::from_iter(iter);
  Ok(v)
}
