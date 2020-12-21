use common::{Day, Part};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-20-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(27798062994017, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-20-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let sections = split_into_sections(data);
  let tiles = load_tiles(&sections);
  let mut neighbors: HashMap<u16, HashSet<u16>> = HashMap::with_capacity(tiles.len());

  for tile in &tiles {
    neighbors.insert(tile.id, HashSet::with_capacity(4));
  }

  for tile in &tiles {
    for edge in tile.edges().iter() {
      for other in &tiles {
        if tile.id == other.id {
          continue;
        }

        for other_edge in other.edges().iter() {
          if edge == other_edge {
            neighbors.get_mut(&tile.id).unwrap().insert(other.id);
          }
        }
      }
    }
  }

  neighbors
    .iter()
    .filter(|(_, ns)| ns.len() == 2)
    .map(|(&id, _)| id as u64)
    .product()
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

struct Tile {
  id: u16,
  pixels: [[char; 10]; 10],
}

impl Tile {
  fn from(lines: &[&str]) -> Tile {
    let regex = Regex::new(r"Tile (\d+):").unwrap();

    if !regex.is_match(lines[0]) {
      panic!("looking for tile header, found '{}'", lines[0]);
    }

    let captures = regex.captures(lines[0]).unwrap();
    let id = u16::from_str(captures.get(1).unwrap().as_str()).unwrap();

    let mut pixels: [[char; 10]; 10] = [['0'; 10]; 10];
    for ix in 0..10 {
      let line = lines[ix + 1];
      for (cx, c) in line.char_indices() {
        pixels[ix][cx] = match c {
          '.' => '0',
          '#' => '1',
          _ => panic!("unexpected character in tile data: '{}'", c),
        }
      }
    }

    Tile { id, pixels }
  }

  fn edges(&self) -> [u16; 8] {
    let str_top: String = self.pixels[0].iter().collect();
    let rts_top: String = self.pixels[0].iter().rev().collect();
    let str_right: String = self.pixels.iter().map(|row| row[9]).collect();
    let rts_right: String = self.pixels.iter().map(|row| row[9]).rev().collect();
    let str_bottom: String = self.pixels[9].iter().collect();
    let rts_bottom: String = self.pixels[9].iter().rev().collect();
    let str_left: String = self.pixels.iter().map(|row| row[0]).collect();
    let rts_left: String = self.pixels.iter().map(|row| row[0]).rev().collect();

    let top_1: u16 = u16::from_str_radix(str_top.as_str(), 2).unwrap();
    let top_2: u16 = u16::from_str_radix(rts_top.as_str(), 2).unwrap();
    let right_1: u16 = u16::from_str_radix(str_right.as_str(), 2).unwrap();
    let right_2: u16 = u16::from_str_radix(rts_right.as_str(), 2).unwrap();
    let bottom_1: u16 = u16::from_str_radix(str_bottom.as_str(), 2).unwrap();
    let bottom_2: u16 = u16::from_str_radix(rts_bottom.as_str(), 2).unwrap();
    let left_1: u16 = u16::from_str_radix(str_left.as_str(), 2).unwrap();
    let left_2: u16 = u16::from_str_radix(rts_left.as_str(), 2).unwrap();

    [
      top_1, right_1, bottom_1, left_1, top_2, right_2, bottom_2, left_2,
    ]
  }
}

fn load_tiles(data: &[&[&str]]) -> Vec<Tile> {
  data.iter().map(|section| Tile::from(section)).collect()
}

fn split_into_sections<'a>(data: &'a [&'a str]) -> Vec<&[&str]> {
  data
    .split(|line| *line == "")
    .filter(|section| section.len() > 0)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_data() -> &'static [&'static str] {
    &[
      "Tile 2311:",
      "..##.#..#.",
      "##..#.....",
      "#...##..#.",
      "####.#...#",
      "##.##.###.",
      "##...#.###",
      ".#.#.#..##",
      "..#....#..",
      "###...#.#.",
      "..###..###",
      "",
      "Tile 1951:",
      "#.##...##.",
      "#.####...#",
      ".....#..##",
      "#...######",
      ".##.#....#",
      ".###.#####",
      "###.##.##.",
      ".###....#.",
      "..#.#..#.#",
      "#...##.#..",
      "",
      "Tile 1171:",
      "####...##.",
      "#..##.#..#",
      "##.#..#.#.",
      ".###.####.",
      "..###.####",
      ".##....##.",
      ".#...####.",
      "#.##.####.",
      "####..#...",
      ".....##...",
      "",
      "Tile 1427:",
      "###.##.#..",
      ".#..#.##..",
      ".#.##.#..#",
      "#.#.#.##.#",
      "....#...##",
      "...##..##.",
      "...#.#####",
      ".#.####.#.",
      "..#..###.#",
      "..##.#..#.",
      "",
      "Tile 1489:",
      "##.#.#....",
      "..##...#..",
      ".##..##...",
      "..#...#...",
      "#####...#.",
      "#..#.#.#.#",
      "...#.#.#..",
      "##.#...##.",
      "..##.##.##",
      "###.##.#..",
      "",
      "Tile 2473:",
      "#....####.",
      "#..#.##...",
      "#.##..#...",
      "######.#.#",
      ".#...#.#.#",
      ".#########",
      ".###.#..#.",
      "########.#",
      "##...##.#.",
      "..###.#.#.",
      "",
      "Tile 2971:",
      "..#.#....#",
      "#...###...",
      "#.#.###...",
      "##.##..#..",
      ".#####..##",
      ".#..####.#",
      "#..#.#..#.",
      "..####.###",
      "..#.#.###.",
      "...#.#.#.#",
      "",
      "Tile 2729:",
      "...#.#.#.#",
      "####.#....",
      "..#.#.....",
      "....#..#.#",
      ".##..##.#.",
      ".#.####...",
      "####.#.#..",
      "##.####...",
      "##..#.##..",
      "#.##...##.",
      "",
      "Tile 3079:",
      "#.#.#####.",
      ".#..######",
      "..#.......",
      "######....",
      "####.#..#.",
      ".#...#.##.",
      "#.#####.##",
      "..#.###...",
      "..#.......",
      "..#.###...",
      "",
      "", // my puzzle data has an extra newline....
    ]
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(&sample_data()), 20899048083289);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_tile_from() {
    let sections = split_into_sections(&sample_data());
    let tile = Tile::from(sections[0]);
    assert_eq!(tile.id, 2311);
    assert_eq!(tile.pixels[0][0], '0');
    assert_eq!(tile.pixels[5][5], '1');
  }

  #[test]
  fn test_tile_edges() {
    let sections = split_into_sections(&sample_data());
    let tile = Tile::from(sections[0]);
    let edges = tile.edges();

    assert_eq!(edges[0], 210);
    assert_eq!(edges[1], 89);
    assert_eq!(edges[2], 231);
    assert_eq!(edges[3], 498);
    assert_eq!(edges[4], 300);
    assert_eq!(edges[5], 616);
    assert_eq!(edges[6], 924);
    assert_eq!(edges[7], 318);
  }

  #[test]
  fn test_load_tiles() {
    let sections = split_into_sections(&sample_data());
    let tiles = load_tiles(&sections);
    assert_eq!(tiles.len(), 9);
  }

  #[test]
  fn test_split_into_sections() {
    let sections = split_into_sections(&sample_data());
    assert_eq!(sections.len(), 9);
  }
}
