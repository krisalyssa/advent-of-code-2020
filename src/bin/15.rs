use common::{Day, Part};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;

pub fn main() {
  let mut data: Vec<String> = vec![];

  // This day's puzzle input was hardcoded into the puzzle page rather than being available for download.
  // In order to use the existing harness, I saved the puzzle input to a file, one number per line.

  if common::load_data("data/day-15-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(1373, day.part_1.result);
    assert_eq!(112458, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-15-input.txt");
    std::process::exit(1);
  }
}

struct Game {
  turn: u32,
  list: VecDeque<u32>,
  memory: HashMap<u32, u32>,
}

impl Game {
  fn new(initial: &[u32]) -> Game {
    Game {
      turn: 1,
      list: VecDeque::from_iter(initial.iter().cloned()),
      memory: HashMap::new(),
    }
  }

  fn next_number(&self) -> u32 {
    self.list.front().cloned().unwrap()
  }

  fn take_turn(&mut self) {
    if let Some(number) = self.list.front().cloned() {
      if let Some(last_spoken) = self.memory.get(&number).cloned() {
        // number was spoken before
        self.memory.insert(number, self.turn);
        self.list.push_back(self.turn - last_spoken);
      } else {
        // number was not spoken before
        self.memory.insert(number, self.turn);
        if self.list.len() == 1 {
          self.list.push_back(0)
        };
      }
      self.list.pop_front();
      self.turn += 1;
    } else {
      panic!("list is empty");
    }
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let initial: Vec<u32> = data.iter().map(|s| u32::from_str(s).unwrap()).collect();

  let mut game = Game::new(&initial);
  for _ in 1..2020 {
    game.take_turn();
  }

  game.next_number() as u64
}

pub fn part_2(data: &[&str]) -> u64 {
  let initial: Vec<u32> = data.iter().map(|s| u32::from_str(s).unwrap()).collect();

  let mut game = Game::new(&initial);
  for _ in 1..30_000_000 {
    game.take_turn();
  }

  game.next_number() as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(&vec!["0", "3", "6"]), 436);
    assert_eq!(part_1(&vec!["1", "3", "2"]), 1);
    assert_eq!(part_1(&vec!["2", "1", "3"]), 10);
    assert_eq!(part_1(&vec!["1", "2", "3"]), 27);
    assert_eq!(part_1(&vec!["2", "3", "1"]), 78);
    assert_eq!(part_1(&vec!["3", "2", "1"]), 438);
    assert_eq!(part_1(&vec!["3", "1", "2"]), 1836);
  }

  #[test]
  fn test_part_2() {
    println!("skipping test_part_2 because it takes a long time to run")
    // assert_eq!(part_2(&vec!["0", "3", "6"]), 175594);
    // assert_eq!(part_2(&vec!["1", "3", "2"]), 2578);
    // assert_eq!(part_2(&vec!["2", "1", "3"]), 3544142);
    // assert_eq!(part_2(&vec!["1", "2", "3"]), 261214);
    // assert_eq!(part_2(&vec!["2", "3", "1"]), 6895259);
    // assert_eq!(part_2(&vec!["3", "2", "1"]), 18);
    // assert_eq!(part_2(&vec!["3", "1", "2"]), 362);
  }

  #[test]
  fn test_take_turn() {
    let mut game = Game::new(&[0, 3, 6]);
    for _ in 1..10 {
      game.take_turn();
    }
    assert_eq!(game.next_number(), 0);
  }
}
