use common::{Day, Part};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-14-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(7997531787333, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-14-input.txt");
    std::process::exit(1);
  }
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
  Mask(String),
  Mem(u32, String),
}

struct PortComputer {
  mask: String,
  mem: HashMap<u32, String>,
}

impl PortComputer {
  fn new() -> PortComputer {
    PortComputer {
      mask: "000000000000000000000000000000000000".to_string(),
      mem: HashMap::new(),
    }
  }

  fn execute(&mut self, program: &Vec<Instruction>) {
    for instruction in program {
      match instruction {
        Instruction::Mask(mask) => self.exec_mask(&mask),
        Instruction::Mem(address, value) => self.exec_mem(*address, &value),
      }
    }
  }

  fn exec_mask(&mut self, mask: &String) {
    self.mask = mask.to_string();
  }

  fn exec_mem(&mut self, address: u32, value: &String) {
    self.mem.insert(address, mask_bits(value, &self.mask));
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut pc = PortComputer::new();
  let program = compile(data);
  pc.execute(&program);

  pc.mem
    .values()
    .map(|v| u64::from_str_radix(v, 2).unwrap())
    .sum()
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn compile(data: &[&str]) -> Vec<Instruction> {
  let re_mask = Regex::new(r"mask = ([01X]{36})").unwrap();
  let re_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
  let mut program: Vec<Instruction> = vec![];

  for line in data {
    if re_mask.is_match(line) {
      let captures = re_mask.captures(line).unwrap();

      let mask = captures.get(1).unwrap().as_str();

      program.push(Instruction::Mask(mask.to_string()));
    } else if re_mem.is_match(line) {
      let captures = re_mem.captures(line).unwrap();

      let address = u32::from_str(captures.get(1).unwrap().as_str()).unwrap();
      let value = u64::from_str(captures.get(2).unwrap().as_str()).unwrap();
      let bits = format!("{:0>36b}", value);

      program.push(Instruction::Mem(address, bits));
    } else {
      panic!("can't compile line: {}", line);
    }
  }

  program
}

fn mask_bits(value: &str, mask: &str) -> String {
  let mut bits = value.to_string();

  for (ix, bit) in mask.char_indices() {
    if bit == 'X' {
      continue;
    }
    bits.replace_range(ix..=ix, &bit.to_string());
  }

  bits
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
      "mem[8] = 11",
      "mem[7] = 101",
      "mem[8] = 0",
    ];
    assert_eq!(part_1(&data), 165);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_port_computer_compile() {
    let program = compile(&[
      "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
      "mem[8] = 11",
      "mem[7] = 101",
      "mem[8] = 0",
    ]);
    assert_eq!(
      *program.get(0).unwrap(),
      Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
    );
    assert_eq!(
      *program.get(1).unwrap(),
      Instruction::Mem(8, "000000000000000000000000000000001011".to_string()),
    );
    assert_eq!(
      *program.get(2).unwrap(),
      Instruction::Mem(7, "000000000000000000000000000001100101".to_string()),
    );
    assert_eq!(
      *program.get(3).unwrap(),
      Instruction::Mem(8, "000000000000000000000000000000000000".to_string()),
    );
  }

  #[test]
  fn test_port_computer_execute() {
    let mut pc = PortComputer::new();
    let program = compile(&[
      "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
      "mem[8] = 11",
      "mem[7] = 101",
      "mem[8] = 0",
    ]);
    pc.execute(&program);
    assert_eq!(pc.mask, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!(
      pc.mem.get(&7).unwrap(),
      "000000000000000000000000000001100101"
    );
    assert_eq!(
      pc.mem.get(&8).unwrap(),
      "000000000000000000000000000001000000"
    );
  }

  #[test]
  fn test_mask_bits() {
    assert_eq!(mask_bits("0000", &"XXXX"), "0000");
    assert_eq!(mask_bits("1111", &"XXXX"), "1111");
    assert_eq!(mask_bits("0000", &"X10X"), "0100");
    assert_eq!(mask_bits("1111", &"X10X"), "1101");
  }
}
