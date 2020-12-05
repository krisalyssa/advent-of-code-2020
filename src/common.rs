use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

//*********************************************************
// related to running days
//

pub type PartFn = fn(&[&str]) -> u64;

pub struct Part {
  fun: PartFn,
  pub result: u64,
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

  pub fn run(&mut self, data: &[String]) {
    let data_as_strs: Vec<&str> = data.iter().map(|v| v.as_str()).collect();

    let start_part_1 = Instant::now();
    self.part_1.result = (self.part_1.fun)(&data_as_strs);
    self.part_1.duration = start_part_1.elapsed();
    let start_part_2 = Instant::now();
    self.part_2.result = (self.part_2.fun)(&data_as_strs);
    self.part_2.duration = start_part_2.elapsed();
    self.duration = start_part_1.elapsed();
  }
}

impl fmt::Display for Day {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let (value_1, units_1) = scale_duration(self.part_1.duration);
    let (value_2, units_2) = scale_duration(self.part_2.duration);
    let (value_total, units_total) = scale_duration(self.duration);

    f.write_fmt(format_args!(
      "Time: part 1 = {} {}, part 2 = {} {}, total = {} {}",
      value_1, units_1, value_2, units_2, value_total, units_total
    ))
  }
}

fn scale_duration(duration: Duration) -> (u128, &'static str) {
  let nanos = duration.as_nanos();

  if nanos >= 1_000_000_000 {
    (duration.as_secs().into(), "s")
  } else if nanos >= 1_000_000 {
    (duration.as_millis(), "ms")
  } else if nanos >= 1_000 {
    (duration.as_micros(), "µs")
  } else {
    (nanos, "ns")
  }
}

pub fn load_data<'a>(filename: &str, data: &'a mut Vec<String>) -> io::Result<&'a [String]> {
  let f = File::open(filename)?;
  load_data_from_reader(f, data)
}

fn load_data_from_reader<R: Read>(raw_reader: R, data: &mut Vec<String>) -> io::Result<&[String]> {
  let reader = BufReader::new(raw_reader);
  for line in reader.lines() {
    match line {
      Ok(v) => data.push(v),
      Err(e) => return Err(e),
    }
  }
  Ok(data)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_data_from_reader() {
    let raw_data = String::from("one\ntwo\nthree");
    let mut data = vec![];

    match load_data_from_reader(raw_data.as_bytes(), &mut data) {
      Ok(v) => assert_eq!(
        (*v).iter().map(String::from).collect::<Vec<String>>(),
        vec!["one", "two", "three"]
          .iter()
          .map(|item| item.to_string())
          .collect::<Vec<String>>()
      ),
      Err(e) => panic!("returned error: {:?}", e),
    }
  }
}
