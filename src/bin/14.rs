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
    assert_eq!(3564822193820, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-14-input.txt");
    std::process::exit(1);
  }
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
  Mask(String),
  Mem(String, String),
}

struct PortComputerMk1 {
  mask: String,
  mem: HashMap<String, String>,
}

impl PortComputerMk1 {
  fn new() -> PortComputerMk1 {
    PortComputerMk1 {
      mask: "000000000000000000000000000000000000".to_string(),
      mem: HashMap::new(),
    }
  }

  fn execute(&mut self, program: &[Instruction]) {
    for instruction in program {
      match instruction {
        Instruction::Mask(mask) => self.exec_mask(&mask),
        Instruction::Mem(address, value) => self.exec_mem(&address, &value),
      }
    }
  }

  fn exec_mask(&mut self, mask: &str) {
    self.mask = mask.to_string();
  }

  fn exec_mem(&mut self, address: &str, value: &str) {
    self
      .mem
      .insert(address.to_owned(), mask_mem_value(value, &self.mask));
  }
}

struct PortComputerMk2 {
  mask: String,
  mem: HashMap<String, String>,
}

impl PortComputerMk2 {
  fn new() -> PortComputerMk2 {
    PortComputerMk2 {
      mask: "000000000000000000000000000000000000".to_string(),
      mem: HashMap::new(),
    }
  }

  fn execute(&mut self, program: &[Instruction]) {
    for instruction in program {
      match instruction {
        Instruction::Mask(mask) => self.exec_mask(&mask),
        Instruction::Mem(address, value) => self.exec_mem(&address, &value),
      }
    }
  }

  fn exec_mask(&mut self, mask: &str) {
    self.mask = mask.to_string();
  }

  fn exec_mem(&mut self, address: &str, value: &str) {
    for addr in floating_addresses(&mask_address(address, &self.mask)) {
      self.mem.insert(addr.to_string(), value.to_string());
    }
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut pc = PortComputerMk1::new();
  let program = compile(data);
  pc.execute(&program);

  pc.mem
    .values()
    .map(|v| u64::from_str_radix(v, 2).unwrap())
    .sum()
}

pub fn part_2(data: &[&str]) -> u64 {
  let mut pc = PortComputerMk2::new();
  let program = compile(data);
  pc.execute(&program);

  pc.mem
    .values()
    .map(|v| u64::from_str_radix(v, 2).unwrap())
    .sum()
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

      let address = u64::from_str(captures.get(1).unwrap().as_str()).unwrap();
      let address_bits = format!("{:0>36b}", address);

      let value = u64::from_str(captures.get(2).unwrap().as_str()).unwrap();
      let value_bits = format!("{:0>36b}", value);

      program.push(Instruction::Mem(address_bits, value_bits));
    } else {
      panic!("can't compile line: {}", line);
    }
  }

  program
}

fn floating_addresses(address: &str) -> Vec<String> {
  if let Some(ix) = address.find('X') {
    let mut bits_0 = address.to_string();
    bits_0.replace_range(ix..=ix, "0");

    let mut bits_1 = address.to_string();
    bits_1.replace_range(ix..=ix, "1");

    vec![floating_addresses(&bits_0), floating_addresses(&bits_1)]
      .into_iter()
      .flatten()
      .collect()
  } else {
    vec![address.to_owned()]
  }
}

fn mask_address(address: &str, mask: &str) -> String {
  let mut bits = address.to_string();

  for (ix, bit) in mask.char_indices() {
    if bit == '0' {
      continue;
    }
    bits.replace_range(ix..=ix, &bit.to_string())
  }

  bits
}

fn mask_mem_value(value: &str, mask: &str) -> String {
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
    let data = vec![
      "mask = 000000000000000000000000000000X1001X",
      "mem[42] = 100",
      "mask = 00000000000000000000000000000000X0XX",
      "mem[26] = 1",
    ];
    assert_eq!(part_2(&data), 208);
  }

  #[test]
  fn test_port_computer_mk1_execute() {
    let mut pc = PortComputerMk1::new();
    let program = compile(&[
      "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
      "mem[8] = 11",
      "mem[7] = 101",
      "mem[8] = 0",
    ]);
    pc.execute(&program);

    assert_eq!(pc.mask, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    assert_eq!(
      pc.mem
        .get(&"000000000000000000000000000000000111".to_string())
        .unwrap(),
      "000000000000000000000000000001100101"
    );
    assert_eq!(
      pc.mem
        .get(&"000000000000000000000000000000001000".to_string())
        .unwrap(),
      "000000000000000000000000000001000000"
    );
  }

  #[test]
  fn test_port_computer_mk2_execute() {
    let mut pc = PortComputerMk2::new();
    let program = compile(&[
      "mask = 000000000000000000000000000000X1001X",
      "mem[42] = 100",
      "mask = 00000000000000000000000000000000X0XX",
      "mem[26] = 1",
    ]);
    pc.execute(&program);

    for address in vec![
      // these two get overwritten in the next write
      // "000000000000000000000000000000011010".to_string(),
      // "000000000000000000000000000000011011".to_string(),
      "000000000000000000000000000000111010".to_string(),
      "000000000000000000000000000000111011".to_string(),
    ] {
      assert_eq!(
        pc.mem.get(&address).unwrap(),
        "000000000000000000000000000001100100"
      );
    }

    for address in vec![
      "000000000000000000000000000000010000".to_string(),
      "000000000000000000000000000000010001".to_string(),
      "000000000000000000000000000000010010".to_string(),
      "000000000000000000000000000000010011".to_string(),
      "000000000000000000000000000000011000".to_string(),
      "000000000000000000000000000000011001".to_string(),
      "000000000000000000000000000000011010".to_string(),
      "000000000000000000000000000000011011".to_string(),
    ] {
      assert_eq!(
        pc.mem.get(&address).unwrap(),
        "000000000000000000000000000000000001"
      );
    }
  }

  #[test]
  fn test_compile() {
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
      Instruction::Mem(
        "000000000000000000000000000000001000".to_string(),
        "000000000000000000000000000000001011".to_string()
      ),
    );
    assert_eq!(
      *program.get(2).unwrap(),
      Instruction::Mem(
        "000000000000000000000000000000000111".to_string(),
        "000000000000000000000000000001100101".to_string()
      ),
    );
    assert_eq!(
      *program.get(3).unwrap(),
      Instruction::Mem(
        "000000000000000000000000000000001000".to_string(),
        "000000000000000000000000000000000000".to_string()
      ),
    );
  }

  #[test]
  fn test_floating_addresses() {
    assert_eq!(
      floating_addresses(&"000000000000000000000000000000010010".to_string()),
      vec!["000000000000000000000000000000010010".to_string()]
    );
    assert_eq!(
      floating_addresses(&"000000000000000000000000000000X1001X".to_string()),
      vec![
        "000000000000000000000000000000010010".to_string(),
        "000000000000000000000000000000010011".to_string(),
        "000000000000000000000000000000110010".to_string(),
        "000000000000000000000000000000110011".to_string()
      ]
    );
    assert_eq!(
      floating_addresses(&"00000000000000000000000000000001X0XX".to_string()),
      vec![
        "000000000000000000000000000000010000".to_string(),
        "000000000000000000000000000000010001".to_string(),
        "000000000000000000000000000000010010".to_string(),
        "000000000000000000000000000000010011".to_string(),
        "000000000000000000000000000000011000".to_string(),
        "000000000000000000000000000000011001".to_string(),
        "000000000000000000000000000000011010".to_string(),
        "000000000000000000000000000000011011".to_string(),
      ]
    );
  }

  #[test]
  fn test_mask_address() {
    assert_eq!(mask_address("0000", &"0000"), "0000");
    assert_eq!(mask_address("1111", &"0000"), "1111");
    assert_eq!(mask_address("0000", &"1111"), "1111");
    assert_eq!(mask_address("1111", &"1111"), "1111");
    assert_eq!(mask_address("0000", &"1X0X"), "1X0X");
    assert_eq!(mask_address("1111", &"1X0X"), "1X1X");
  }

  #[test]
  fn test_mask_mem_value() {
    assert_eq!(mask_mem_value("0000", &"XXXX"), "0000");
    assert_eq!(mask_mem_value("1111", &"XXXX"), "1111");
    assert_eq!(mask_mem_value("0000", &"X10X"), "0100");
    assert_eq!(mask_mem_value("1111", &"X10X"), "1101");
  }
}
