use common::{Day, Part};
use std::collections::HashSet;
use std::str::FromStr;

type Operation<'a> = &'a str;
type Argument = i32;
type Instruction<'a> = (Operation<'a>, Argument);

struct Device<'a> {
  ram: Vec<Instruction<'a>>,
  pc: usize,
  acc: Argument,
  visited_locations: HashSet<usize>,
  instructions_executed: u32,
  instructions_limit: Option<u32>,
  trace_enabled: bool,
}

const ACC: &str = "acc";
const HLT: &str = "hlt";
const JMP: &str = "jmp";
const NOP: &str = "nop";

impl<'a> Device<'a> {
  pub fn from_slice(image: &'a [&'a str]) -> Device<'a> {
    let parsed_image = image
      .iter()
      .map(|line| {
        // println!("parsing {}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();
        let operation = parts[0];
        let argument = Argument::from_str(parts[1]).unwrap();
        (operation, argument)
      })
      .collect();
    Device {
      ram: parsed_image,
      pc: 0,
      acc: 0,
      visited_locations: HashSet::new(),
      instructions_executed: 0,
      instructions_limit: None,
      trace_enabled: false,
    }
  }

  pub fn peek(&self, addr: usize) -> Instruction {
    if let Some(value) = self.ram.get(addr) {
      *value
    } else {
      panic!(
        "memory out-of-bounds exception: tried to read from {}; valid range is 0..{}",
        addr,
        self.ram.len() - 1
      );
    }
  }

  pub fn poke(&mut self, addr: usize, value: Instruction<'a>) {
    if let Some(ptr) = self.ram.get_mut(addr) {
      *ptr = value;
    } else {
      panic!(
        "memory out-of-bounds exception: tried to write to {}; valid range is 0..{}",
        addr,
        self.ram.len()
      );
    }
  }

  pub fn run(&mut self) -> (Option<Operation>, Argument) {
    let mut last_operation = None;

    loop {
      if let Some(limit) = self.instructions_limit {
        self.instructions_executed += 1;
        if self.instructions_executed > limit {
          if self.trace_enabled {
            eprintln!(
              "execution halted after {} instructions: pc = {}, acc = {}",
              limit, self.pc, self.acc
            );
          }
          break;
        }
      }

      if !self.visited_locations.insert(self.pc) {
        if self.trace_enabled {
          eprintln!(
            "execution halted because instruction was revisited: pc = {}, acc = {}",
            self.pc, self.acc
          );
        }
        break;
      }

      let (operation, argument) = self.fetch_instruction();
      match operation {
        ACC => self.op_acc(argument),
        HLT => {
          last_operation = Some(HLT);
          break;
        }
        JMP => self.op_jmp(argument),
        NOP => self.op_nop(argument),
        _ => panic!("unknown operation: {}", operation),
      }
    }

    (last_operation, self.acc)
  }

  fn fetch_instruction(&self) -> Instruction {
    if self.pc >= self.ram.len() {
      (HLT, 0)
    } else {
      self.peek(self.pc)
    }
  }

  fn op_acc(&mut self, argument: Argument) {
    if self.trace_enabled {
      eprintln!("{}: ACC {}  # {}", self.pc, argument, self.acc + argument);
    }
    self.acc += argument;
    self.pc += 1;
  }

  fn op_jmp(&mut self, argument: Argument) {
    if self.trace_enabled {
      eprintln!(
        "{}: JMP {}  # {}",
        self.pc,
        argument,
        self.pc as i32 + argument
      );
    }
    self.pc = (self.pc as i32 + argument) as usize;
  }

  fn op_nop(&mut self, _argument: Argument) {
    if self.trace_enabled {
      eprintln!("{}: NOP", self.pc);
    }
    self.pc += 1;
  }
}

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-08-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(1675, day.part_1.result);
    assert_eq!(1532, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-08-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut device = Device::from_slice(data);
  if let (None, argument) = device.run() {
    argument as u64
  } else {
    panic!("device did not return a value");
  }
}

pub fn part_2(data: &[&str]) -> u64 {
  let mut retval = None;

  for (ix, _) in data.iter().enumerate() {
    let mut device = Device::from_slice(data);

    match device.peek(ix) {
      (JMP, argument) => device.poke(ix, (NOP, argument)),
      (NOP, argument) => device.poke(ix, (JMP, argument)),
      _ => continue,
    }

    if let (Some(HLT), argument) = device.run() {
      retval = Some(argument);
      break;
    }
  }

  if let Some(r) = retval {
    r as u64
  } else {
    panic!("device did not return a value");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];
    assert_eq!(part_1(&data), 5);
  }

  #[test]
  fn test_part_2() {
    let data = vec![
      "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];
    assert_eq!(part_2(&data), 8);
  }
}
