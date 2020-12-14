use common::{Day, Part};

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-13-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(2406, day.part_1.result);
    assert_eq!(225850756401039, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-13-input.txt");
    std::process::exit(1);
  }
}

struct Timetable {
  timestamp: u64,
  ids: Vec<Option<u64>>,
}

impl Timetable {
  fn new() -> Timetable {
    Timetable {
      timestamp: 0,
      ids: vec![],
    }
  }

  fn from(data: &[&str]) -> Timetable {
    let mut timetable = Timetable::new();

    let mut iter = data.iter();

    if let Some(timestamp) = iter.next() {
      if let Some(ids) = iter.next() {
        timetable.timestamp = timestamp.parse().ok().unwrap();
        timetable.ids = ids
          .split(',')
          .map(|id| match id {
            "x" => None,
            num => num.parse().ok(),
          })
          .collect();
      }
    }

    timetable
  }

  fn departure_delay_for_bus(&self, id: u64) -> u64 {
    id - (self.timestamp % id)
  }

  fn remainders(&self) -> Vec<(u64, u64)> {
    self
      .ids
      .iter()
      .enumerate()
      .filter(|(_, id)| id.is_some())
      .map(|(ix, id)| {
        (
          id.unwrap(),
          normalized_mod((id.unwrap() - ix as u64) as i64, id.unwrap() as i64) as u64,
        )
      })
      .collect()
  }

  fn shortest_delay(&self) -> (u64, u64) {
    self
      .ids
      .iter()
      .filter(|id| id.is_some())
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

pub fn part_2(data: &[&str]) -> u64 {
  let timetable = Timetable::from(&data);
  let remainders = timetable.remainders();
  let residues = residues(&remainders);
  let modulii = modulii(&remainders);

  chinese_remainder(&residues, &modulii).unwrap() as u64
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
  let prod = modulii.iter().product::<i64>();
  let mut sum = 0;

  for (&residue, &modulus) in residues.iter().zip(modulii) {
    let p = prod / modulus;
    sum += residue * mmi(p, modulus)? * p
  }

  Some(sum % prod)
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
  if a == 0 {
    (b, 0, 1)
  } else {
    let (g, mmi_a, mmi_b) = extended_euclidean(b % a, a);
    (g, mmi_b - (b / a) * mmi_a, mmi_a)
  }
}

fn mmi(x: i64, n: i64) -> Option<i64> {
  let (g, x, _) = extended_euclidean(x, n);
  if g == 1 {
    Some((x % n + n) % n)
  } else {
    None
  }
}

fn modulii(pairs: &[(u64, u64)]) -> Vec<i64> {
  pairs.iter().map(|(modulus, _)| *modulus as i64).collect()
}

fn normalized_mod(dividend: i64, modulus: i64) -> i64 {
  if dividend > 0 {
    dividend % modulus
  } else {
    normalized_mod(dividend + modulus, modulus)
  }
}

fn residues(pairs: &[(u64, u64)]) -> Vec<i64> {
  pairs
    .iter()
    .map(|(_, remainder)| *remainder as i64)
    .collect()
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
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    assert_eq!(part_2(&data), 1068781);
  }

  #[test]
  fn test_chinese_remainder() {
    assert_eq!(chinese_remainder(&[2, 3, 2], &[3, 5, 7]), Some(23));
    assert_eq!(chinese_remainder(&[1, 2, 1], &[3, 5, 7]), Some(22));
    assert_eq!(chinese_remainder(&[0, 1, 0], &[3, 5, 7]), Some(21));
    assert_eq!(
      chinese_remainder(&[0, 12, 55, 25, 12], &[7, 13, 59, 31, 19]),
      Some(1068781)
    );
    assert_eq!(chinese_remainder(&[0, 11, 16], &[17, 13, 19]), Some(3417));
    assert_eq!(
      chinese_remainder(&[0, 6, 57, 58], &[67, 7, 59, 61]),
      Some(754018)
    );
    assert_eq!(
      chinese_remainder(&[0, 5, 56, 57], &[67, 7, 59, 61]),
      Some(779210)
    );
    assert_eq!(
      chinese_remainder(&[0, 6, 56, 57], &[67, 7, 59, 61]),
      Some(1261476)
    );
    assert_eq!(
      chinese_remainder(&[0, 36, 45, 1886], &[1789, 37, 47, 1889]),
      Some(1202161486)
    );

    assert_eq!(
      chinese_remainder(
        &[0, 28, 486, 3, 1, 6, 347, 14, 3],
        &[23, 41, 509, 13, 17, 29, 401, 37, 19]
      ),
      Some(786980394408406)
    );
  }

  #[test]
  fn test_normalized_mod() {
    assert_eq!(normalized_mod(10, 7), 3);
    assert_eq!(normalized_mod(-10, 7), 4);
    assert_eq!(normalized_mod(-17, 7), 4);
  }

  #[test]
  fn test_timetable_from() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(timetable.timestamp, 939);
    assert_eq!(
      timetable.ids,
      vec![
        Some(7),
        Some(13),
        None,
        None,
        Some(59),
        None,
        Some(31),
        Some(19)
      ]
    );
  }

  #[test]
  fn test_timetable_departure_delay_for_bus() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(timetable.departure_delay_for_bus(7), 6);
    assert_eq!(timetable.departure_delay_for_bus(13), 10);
    assert_eq!(timetable.departure_delay_for_bus(59), 5);
    assert_eq!(timetable.departure_delay_for_bus(31), 22);
    assert_eq!(timetable.departure_delay_for_bus(19), 11);
  }

  #[test]
  fn test_timetable_remainders() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(
      timetable.remainders(),
      vec![(7, 0), (13, 12), (59, 55), (31, 25), (19, 12)]
    );
  }

  #[test]
  fn test_timetable_shortest_delay() {
    let data = vec!["939", "7,13,x,x,59,x,31,19"];
    let timetable = Timetable::from(&data);
    assert_eq!(timetable.shortest_delay(), (59, 5));
  }
}
