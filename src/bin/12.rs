use common::{Day, Part};
use regex::Regex;
use std::fmt;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-12-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(362, day.part_1.result);
    assert_eq!(29895, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-12-input.txt");
    std::process::exit(1);
  }
}

enum Command {
  North(i32),
  South(i32),
  East(i32),
  West(i32),
  Left(i32),
  Right(i32),
  Forward(i32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
  lat: i32,
  lon: i32,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.lat, self.lon)
  }
}

impl Point {
  fn new(lat: i32, lon: i32) -> Point {
    Point { lat, lon }
  }

  fn origin() -> Point {
    Point { lat: 0, lon: 0 }
  }

  fn manhattan_distance_from(&self, other: Point) -> i32 {
    (self.lat - other.lat).abs() + (self.lon - other.lon).abs()
  }

  fn rotate_around(&mut self, other: &Point, angle: i32) {
    let delta_lat = self.lat - other.lat;
    let delta_lon = self.lon - other.lon;
    match normalize_angle(angle) {
      0 => {}
      90 => {
        self.lat = other.lat - delta_lon;
        self.lon = other.lon + delta_lat;
      }
      180 => {
        self.lat = other.lat - delta_lat;
        self.lon = other.lon - delta_lon;
      }
      270 => {
        self.lat = other.lat + delta_lon;
        self.lon = other.lon - delta_lat;
      }
      angle => panic!("unexpected angle: {}", angle),
    }
  }
}

struct ShipMk1 {
  location: Point,
  heading: i32,
}

impl ShipMk1 {
  fn new() -> ShipMk1 {
    ShipMk1 {
      location: Point::new(0, 0),
      heading: 90,
    }
  }

  fn execute(&mut self, command: Command) {
    match command {
      Command::North(distance) => self.location.lat += distance,
      Command::South(distance) => self.location.lat -= distance,
      Command::East(distance) => self.location.lon += distance,
      Command::West(distance) => self.location.lon -= distance,
      Command::Left(angle) => self.heading = normalize_angle(self.heading - angle),
      Command::Right(angle) => self.heading = normalize_angle(self.heading + angle),
      Command::Forward(distance) => match self.heading {
        0 => self.execute(Command::North(distance)),
        90 => self.execute(Command::East(distance)),
        180 => self.execute(Command::South(distance)),
        270 => self.execute(Command::West(distance)),
        angle => panic!("unexpected heading: {}", angle),
      },
    }
  }
}

struct ShipMk2 {
  location: Point,
  waypoint: Point,
}

impl ShipMk2 {
  fn new() -> ShipMk2 {
    ShipMk2 {
      location: Point::new(0, 0),
      waypoint: Point::new(1, 10),
    }
  }

  fn execute(&mut self, command: Command) {
    match command {
      Command::North(distance) => self.waypoint.lat += distance,
      Command::South(distance) => self.waypoint.lat -= distance,
      Command::East(distance) => self.waypoint.lon += distance,
      Command::West(distance) => self.waypoint.lon -= distance,
      Command::Left(angle) => self.waypoint.rotate_around(&Point::origin(), -angle),
      Command::Right(angle) => self.waypoint.rotate_around(&Point::origin(), angle),
      Command::Forward(distance) => {
        self.location.lat += self.waypoint.lat * distance;
        self.location.lon += self.waypoint.lon * distance;
      }
    }
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut ship = ShipMk1::new();

  for line in data {
    if let Some(command) = interpret_command_line(line) {
      ship.execute(command);
    }
  }

  ship.location.manhattan_distance_from(Point::new(0, 0)) as u64
}

pub fn part_2(data: &[&str]) -> u64 {
  let mut ship = ShipMk2::new();

  for line in data {
    if let Some(command) = interpret_command_line(line) {
      ship.execute(command);
    }
  }

  ship.location.manhattan_distance_from(Point::new(0, 0)) as u64
}

fn interpret_command_line(line: &str) -> Option<Command> {
  let regex = Regex::new(r"([EFLNRSW])(\d+)").unwrap();

  if let Some(captures) = regex.captures(line) {
    let command_selector: &str = captures.get(1).unwrap().as_str();
    let amount: i32 = captures.get(2).unwrap().as_str().parse().ok().unwrap();

    let command = match command_selector {
      "N" => Command::North(amount),
      "S" => Command::South(amount),
      "E" => Command::East(amount),
      "W" => Command::West(amount),
      "L" => Command::Left(amount),
      "R" => Command::Right(amount),
      "F" => Command::Forward(amount),
      other => panic!("unexpected command selector: {}", other),
    };

    Some(command)
  } else {
    None
  }
}

fn normalize_angle(angle: i32) -> i32 {
  let mut new_angle = angle;
  while new_angle < 0 {
    new_angle += 360;
  }
  new_angle % 360
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec!["F10", "N3", "F7", "R90", "F11"];
    assert_eq!(part_1(&data), 25);
  }

  #[test]
  fn test_part_2() {
    let data = vec!["F10", "N3", "F7", "R90", "F11"];
    assert_eq!(part_2(&data), 286);
  }

  #[test]
  fn test_point_manhattan_distance() {
    let point = Point::new(0, 0);
    assert_eq!(point.manhattan_distance_from(Point::new(17, 8)), 25);
    assert_eq!(point.manhattan_distance_from(Point::new(17, -8)), 25);
    assert_eq!(point.manhattan_distance_from(Point::new(-17, 8)), 25);
    assert_eq!(point.manhattan_distance_from(Point::new(-17, -8)), 25);
  }

  #[test]
  fn test_point_rotate_around() {
    let mut point = Point::new(4, 10);
    let origin = Point::new(0, 0);

    point.rotate_around(&origin, 90);
    assert_eq!(point, Point::new(-10, 4));

    point.rotate_around(&origin, -90);
    assert_eq!(point, Point::new(4, 10));

    point.rotate_around(&origin, 180);
    assert_eq!(point, Point::new(-4, -10));

    point.rotate_around(&origin, 0);
    assert_eq!(point, Point::new(-4, -10));
  }
}
