use common::{Day, Part};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-17-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(319, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-17-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut grid: HashSet<(i64, i64, i64)> = HashSet::new();
  initialize_space(&data, &mut grid);

  for _ in 0..6 {
    step(&mut grid);
  }

  grid.len() as u64
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

fn count_neighbors(grid: &HashSet<(i64, i64, i64)>, cell: &(i64, i64, i64)) -> u8 {
  let mut count: u8 = 0;

  for x in (cell.0 - 1)..=(cell.0 + 1) {
    for y in (cell.1 - 1)..=(cell.1 + 1) {
      for z in (cell.2 - 1)..=(cell.2 + 1) {
        if x == cell.0 && y == cell.1 && z == cell.2 {
          continue;
        }
        if grid.contains(&(x, y, z)) {
          count += 1
        }
      }
    }
  }

  count
}

fn extents(
  grid: &HashSet<(i64, i64, i64)>,
) -> (
  RangeInclusive<i64>,
  RangeInclusive<i64>,
  RangeInclusive<i64>,
) {
  let ((min_x, _, _), (max_x, _, _)) = grid
    .iter()
    .minmax_by(|(x1, _, _), (x2, _, _)| x1.cmp(x2))
    .into_option()
    .unwrap();
  let ((_, min_y, _), (_, max_y, _)) = grid
    .iter()
    .minmax_by(|(_, y1, _), (_, y2, _)| y1.cmp(y2))
    .into_option()
    .unwrap();
  let ((_, _, min_z), (_, _, max_z)) = grid
    .iter()
    .minmax_by(|(_, _, z1), (_, _, z2)| z1.cmp(z2))
    .into_option()
    .unwrap();
  (*min_x..=*max_x, *min_y..=*max_y, *min_z..=*max_z)
}

fn initialize_space(data: &[&str], grid: &mut HashSet<(i64, i64, i64)>) {
  for (y, row) in data.iter().enumerate() {
    for (x, col) in row.chars().enumerate() {
      if col == '#' {
        grid.insert((x as i64, y as i64, 0));
      }
    }
  }
}

fn step(grid: &mut HashSet<(i64, i64, i64)>) {
  let previous: HashSet<(i64, i64, i64)> = HashSet::from_iter(grid.iter().copied());
  let (range_x, range_y, range_z) = extents(&previous);

  for x in range_x.start() - 1..=range_x.end() + 1 {
    for y in range_y.start() - 1..=range_y.end() + 1 {
      for z in range_z.start() - 1..=range_z.end() + 1 {
        let previously_active = previous.contains(&(x, y, z));
        let count = count_neighbors(&previous, &(x, y, z));

        if previously_active && !(2..=3).contains(&count) {
          grid.remove(&(x, y, z));
        } else if !previously_active && count == 3 {
          grid.insert((x, y, z));
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![".#.", "..#", "###"];
    assert_eq!(part_1(&data), 112);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_count_neighbors() {
    let mut grid: HashSet<(i64, i64, i64)> = HashSet::new();
    assert_eq!(count_neighbors(&grid, &(0, 0, 0)), 0);

    let data = vec![".#.", "..#", "###"];
    initialize_space(&data, &mut grid);
    assert_eq!(count_neighbors(&grid, &(0, 0, 0)), 1);
    assert_eq!(count_neighbors(&grid, &(1, 2, 0)), 3);
  }

  #[test]
  fn test_extents() {
    let data = vec![".#.", "..##", "###"];
    let mut grid: HashSet<(i64, i64, i64)> = HashSet::new();
    initialize_space(&data, &mut grid);
    let (range_x, range_y, range_z) = extents(&grid);
    assert_eq!(range_x, (0..=3));
    assert_eq!(range_y, (0..=2));
    assert_eq!(range_z, (0..=0));
  }

  #[test]
  fn test_initialize_space() {
    let data = vec![".#.", "..#", "###"];
    let mut grid: HashSet<(i64, i64, i64)> = HashSet::new();
    initialize_space(&data, &mut grid);
    assert_eq!(grid.len(), 5);
    assert!(grid.contains(&(1, 0, 0)));
    assert!(grid.contains(&(2, 1, 0)));
    assert!(grid.contains(&(0, 2, 0)));
    assert!(grid.contains(&(1, 2, 0)));
    assert!(grid.contains(&(2, 2, 0)));
  }

  #[test]
  fn test_step() {
    let data = vec![".#.", "..#", "###"];
    let mut grid: HashSet<(i64, i64, i64)> = HashSet::new();
    initialize_space(&data, &mut grid);

    step(&mut grid);
    // println!("{:?}", grid);
    assert_eq!(grid.len(), 11);
  }
}
