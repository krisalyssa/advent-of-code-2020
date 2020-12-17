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
    assert_eq!(2324, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-17-input.txt");
    std::process::exit(1);
  }
}

type Point = (i64, i64, i64, i64);
type Extents = (
  RangeInclusive<i64>,
  RangeInclusive<i64>,
  RangeInclusive<i64>,
  RangeInclusive<i64>,
);

pub fn part_1(data: &[&str]) -> u64 {
  let mut grid: HashSet<Point> = HashSet::new();
  initialize_space(&data, &mut grid);

  for _ in 0..6 {
    step_3d(&mut grid);
  }

  grid.len() as u64
}

pub fn part_2(data: &[&str]) -> u64 {
  let mut grid: HashSet<Point> = HashSet::new();
  initialize_space(&data, &mut grid);

  for _ in 0..6 {
    step_4d(&mut grid);
  }

  grid.len() as u64
}

fn count_neighbors_3d(grid: &HashSet<Point>, cell: &Point) -> u8 {
  let mut count: u8 = 0;

  for x in (cell.0 - 1)..=(cell.0 + 1) {
    for y in (cell.1 - 1)..=(cell.1 + 1) {
      for z in (cell.2 - 1)..=(cell.2 + 1) {
        if x == cell.0 && y == cell.1 && z == cell.2 {
          continue;
        }
        if grid.contains(&(x, y, z, 0)) {
          count += 1
        }
      }
    }
  }

  count
}

fn count_neighbors_4d(grid: &HashSet<Point>, cell: &Point) -> u8 {
  let mut count: u8 = 0;

  for x in (cell.0 - 1)..=(cell.0 + 1) {
    for y in (cell.1 - 1)..=(cell.1 + 1) {
      for z in (cell.2 - 1)..=(cell.2 + 1) {
        for w in (cell.3 - 1)..=(cell.3 + 1) {
          if x == cell.0 && y == cell.1 && z == cell.2 && w == cell.3 {
            continue;
          }
          if grid.contains(&(x, y, z, w)) {
            count += 1
          }
        }
      }
    }
  }

  count
}

fn extents(grid: &HashSet<Point>) -> Extents {
  let ((min_x, _, _, _), (max_x, _, _, _)) = grid
    .iter()
    .minmax_by(|(x1, _, _, _), (x2, _, _, _)| x1.cmp(x2))
    .into_option()
    .unwrap();
  let ((_, min_y, _, _), (_, max_y, _, _)) = grid
    .iter()
    .minmax_by(|(_, y1, _, _), (_, y2, _, _)| y1.cmp(y2))
    .into_option()
    .unwrap();
  let ((_, _, min_z, _), (_, _, max_z, _)) = grid
    .iter()
    .minmax_by(|(_, _, z1, _), (_, _, z2, _)| z1.cmp(z2))
    .into_option()
    .unwrap();
  let ((_, _, _, min_w), (_, _, _, max_w)) = grid
    .iter()
    .minmax_by(|(_, _, _, w1), (_, _, _, w2)| w1.cmp(w2))
    .into_option()
    .unwrap();
  (
    *min_x..=*max_x,
    *min_y..=*max_y,
    *min_z..=*max_z,
    *min_w..=*max_w,
  )
}

fn initialize_space(data: &[&str], grid: &mut HashSet<Point>) {
  for (y, row) in data.iter().enumerate() {
    for (x, col) in row.chars().enumerate() {
      if col == '#' {
        grid.insert((x as i64, y as i64, 0, 0));
      }
    }
  }
}

fn step_3d(grid: &mut HashSet<Point>) {
  let previous: HashSet<Point> = HashSet::from_iter(grid.iter().copied());
  let (range_x, range_y, range_z, _range_w) = extents(&previous);

  for x in range_x.start() - 1..=range_x.end() + 1 {
    for y in range_y.start() - 1..=range_y.end() + 1 {
      for z in range_z.start() - 1..=range_z.end() + 1 {
        let previously_active = previous.contains(&(x, y, z, 0));
        let count = count_neighbors_3d(&previous, &(x, y, z, 0));

        if previously_active && !(2..=3).contains(&count) {
          grid.remove(&(x, y, z, 0));
        } else if !previously_active && count == 3 {
          grid.insert((x, y, z, 0));
        }
      }
    }
  }
}

fn step_4d(grid: &mut HashSet<Point>) {
  let previous: HashSet<Point> = HashSet::from_iter(grid.iter().copied());
  let (range_x, range_y, range_z, range_w) = extents(&previous);

  for x in range_x.start() - 1..=range_x.end() + 1 {
    for y in range_y.start() - 1..=range_y.end() + 1 {
      for z in range_z.start() - 1..=range_z.end() + 1 {
        for w in range_w.start() - 1..=range_w.end() + 1 {
          let previously_active = previous.contains(&(x, y, z, w));
          let count = count_neighbors_4d(&previous, &(x, y, z, w));

          if previously_active && !(2..=3).contains(&count) {
            grid.remove(&(x, y, z, w));
          } else if !previously_active && count == 3 {
            grid.insert((x, y, z, w));
          }
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
    let data = vec![".#.", "..#", "###"];
    assert_eq!(part_2(&data), 848);
  }

  #[test]
  fn test_count_neighbors_3d() {
    let mut grid: HashSet<Point> = HashSet::new();
    assert_eq!(count_neighbors_3d(&grid, &(0, 0, 0, 0)), 0);

    let data = vec![".#.", "..#", "###"];
    initialize_space(&data, &mut grid);
    assert_eq!(count_neighbors_3d(&grid, &(0, 0, 0, 0)), 1);
    assert_eq!(count_neighbors_3d(&grid, &(1, 2, 0, 0)), 3);
  }

  #[test]
  fn test_count_neighbors_4d() {
    let mut grid: HashSet<Point> = HashSet::new();
    assert_eq!(count_neighbors_4d(&grid, &(0, 0, 0, 0)), 0);

    let data = vec![".#.", "..#", "###"];
    initialize_space(&data, &mut grid);
    assert_eq!(count_neighbors_4d(&grid, &(0, 0, 0, 0)), 1);
    assert_eq!(count_neighbors_4d(&grid, &(1, 2, 0, 0)), 3);
  }

  #[test]
  fn test_extents() {
    let data = vec![".#.", "..##", "###"];
    let mut grid: HashSet<Point> = HashSet::new();
    initialize_space(&data, &mut grid);
    let (range_x, range_y, range_z, range_w) = extents(&grid);
    assert_eq!(range_x, (0..=3));
    assert_eq!(range_y, (0..=2));
    assert_eq!(range_z, (0..=0));
    assert_eq!(range_w, (0..=0));
  }

  #[test]
  fn test_initialize_space() {
    let data = vec![".#.", "..#", "###"];
    let mut grid: HashSet<Point> = HashSet::new();
    initialize_space(&data, &mut grid);
    assert_eq!(grid.len(), 5);
    assert!(grid.contains(&(1, 0, 0, 0)));
    assert!(grid.contains(&(2, 1, 0, 0)));
    assert!(grid.contains(&(0, 2, 0, 0)));
    assert!(grid.contains(&(1, 2, 0, 0)));
    assert!(grid.contains(&(2, 2, 0, 0)));
  }

  #[test]
  fn test_step_3d() {
    let data = vec![".#.", "..#", "###"];
    let mut grid: HashSet<Point> = HashSet::new();
    initialize_space(&data, &mut grid);

    step_3d(&mut grid);
    // println!("{:?}", grid);
    assert_eq!(grid.len(), 11);
  }

  #[test]
  fn test_step_4d() {
    let data = vec![".#.", "..#", "###"];
    let mut grid: HashSet<Point> = HashSet::new();
    initialize_space(&data, &mut grid);

    step_4d(&mut grid);
    // println!("{:?}", grid);
    assert_eq!(grid.len(), 29);
  }
}
