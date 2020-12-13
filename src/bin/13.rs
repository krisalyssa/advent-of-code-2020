use common::{Day, Part};

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-13-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(2406, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-13-input.txt");
    std::process::exit(1);
  }
}

struct Timetable {
  timestamp: u64,
  buses: Vec<Option<u32>>,
}

impl Timetable {
  fn new() -> Timetable {
    Timetable {
      timestamp: 0,
      buses: vec![],
    }
  }

  fn from(data: &[&str]) -> Timetable {
    let mut timetable = Timetable::new();

    let mut iter = data.iter();

    if let Some(timestamp) = iter.next() {
      if let Some(buses) = iter.next() {
        timetable.timestamp = timestamp.parse().ok().unwrap();
        timetable.buses = buses
          .split(',')
          .filter(|id| *id != "x")
          .map(|id| id.parse().ok())
          .collect();
      }
    }

    timetable
  }

  fn departure_delay_for_bus(&self, id: u32) -> u64 {
    id as u64 - (self.timestamp % id as u64)
  }

  fn shortest_delay(&self) -> (u32, u64) {
    self
      .buses
      .iter()
      .map(|id| (id.unwrap(), self.departure_delay_for_bus(id.unwrap())))
      .min_by(|(_, va), (_, vb)| va.cmp(vb))
      .unwrap()
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let timetable = Timetable::from(&data);
  let (id, delay) = timetable.shortest_delay();
  id as u64 * delay
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    assert_eq!(part_1(&data), 295);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_timetable_from() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(timetable.timestamp, 939);
    assert_eq!(
      timetable.buses,
      vec![Some(7), Some(13), Some(59), Some(31), Some(19)]
    );
  }

  #[test]
  fn test_departure_delay_for_bus() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(timetable.departure_delay_for_bus(7), 6);
    assert_eq!(timetable.departure_delay_for_bus(13), 10);
    assert_eq!(timetable.departure_delay_for_bus(59), 5);
    assert_eq!(timetable.departure_delay_for_bus(31), 22);
    assert_eq!(timetable.departure_delay_for_bus(19), 11);
  }

  #[test]
  fn test_shortest_delay() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(timetable.shortest_delay(), (59, 5));
  }
}
