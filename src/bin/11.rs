use common::{Day, Part};
use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-11-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(2476, day.part_1.result);
    assert_eq!(0, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-11-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let mut room = Room::from(data);

  for _ in 0..100 {
    if !room.step() {
      break;
    }
  }

  room.count_occupied_seats() as u64
}

pub fn part_2(_data: &[&str]) -> u64 {
  0
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cell {
  x: i32,
  y: i32,
  occupied: bool,
}

impl Cell {
  fn new(x: i32, y: i32) -> Cell {
    Cell {
      x: x,
      y: y,
      occupied: false,
    }
  }

  fn null() -> Cell {
    Cell {
      x: 0,
      y: 0,
      occupied: false,
    }
  }
}

#[derive(Debug)]
struct Room {
  width: i32,
  height: i32,
  cells: Vec<Option<Cell>>,
}

impl Room {
  fn new() -> Room {
    Room {
      width: 0,
      height: 0,
      cells: vec![],
    }
  }

  fn from(data: &[&str]) -> Room {
    let mut room = Room::new();
    let height = data.len() as i32;
    let width = data[0].len() as i32;
    room.resize(width, height);

    for (y, line) in data.iter().enumerate() {
      for (x, cell) in line.graphemes(true).enumerate() {
        match cell {
          "L" => room.cells.push(Some(Cell::new(x as i32, y as i32))),
          "." => room.cells.push(None),
          _ => panic!("don't know what to do with '{}'", cell),
        }
      }
    }

    room
  }

  fn changed(&self, next_gen: &Vec<Option<Cell>>) -> bool {
    self
      .cells
      .iter()
      .zip(next_gen)
      .any(|(old, new)| old.unwrap_or(Cell::null()) != new.unwrap_or(Cell::null()))
  }

  fn count_neighbors_of(&self, cell: Cell) -> u8 {
    let deltas: Vec<(i32, i32)> = if cell.x == 0 {
      if cell.y == 0 {
        // top left
        vec![(1, 0), (1, 1), (0, 1)]
      } else if cell.y == self.height - 1 {
        // bottom left
        vec![(0, -1), (1, -1), (1, 0)]
      } else {
        // left edge
        vec![(0, -1), (1, -1), (1, 0), (1, 1), (0, 1)]
      }
    } else if cell.x == self.width - 1 {
      if cell.y == 0 {
        // top right
        vec![(0, 1), (-1, 1), (-1, 0)]
      } else if cell.y == self.height - 1 {
        // bottom right
        vec![(-1, 0), (-1, -1), (0, -1)]
      } else {
        // right edge
        vec![(0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1)]
      }
    } else {
      if cell.y == 0 {
        // top edge
        vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
      } else if cell.y == self.height - 1 {
        // bottom edge
        vec![(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)]
      } else {
        // body
        vec![
          (-1, 0),
          (-1, -1),
          (0, -1),
          (1, -1),
          (1, 0),
          (1, 1),
          (0, 1),
          (-1, 1),
        ]
      }
    };

    deltas.iter().fold(0, |acc, (dx, dy)| {
      acc + self.get_occupied(cell.x + dx, cell.y + dy)
    })
  }

  fn count_occupied_seats(&self) -> u32 {
    self
      .cells
      .iter()
      .filter(|cell| cell.is_some())
      .filter(|cell| (**cell).unwrap().occupied)
      .count() as u32
  }

  fn get(&self, x: i32, y: i32) -> Option<Cell> {
    if x >= self.width {
      None
    } else {
      if let Some(Some(cell_ref)) = self.cells.get(((y * self.width) + x) as usize) {
        Some(*cell_ref)
      } else {
        None
      }
    }
  }

  fn get_occupied(&self, x: i32, y: i32) -> u8 {
    match self.get(x, y) {
      Some(cell) => {
        if cell.occupied {
          1
        } else {
          0
        }
      }
      None => 0,
    }
  }

  fn resize(&mut self, width: i32, height: i32) {
    self.width = width;
    self.height = height;
    self.cells = Vec::with_capacity((width * height) as usize);
  }

  fn step(&mut self) -> bool {
    let mut next_gen: Vec<Option<Cell>> = self.cells.iter().copied().collect();

    for cell in self.cells.iter().filter(|oc| oc.is_some()) {
      if let Some(c) = cell {
        let neighbors = self.count_neighbors_of(*c);
        if !c.occupied && neighbors == 0 {
          next_gen
            .get_mut(((c.y * self.width) + c.x) as usize)
            .unwrap()
            .as_mut()
            .unwrap()
            .occupied = true;
        } else if c.occupied && neighbors >= 4 {
          next_gen
            .get_mut(((c.y * self.width) + c.x) as usize)
            .unwrap()
            .as_mut()
            .unwrap()
            .occupied = false;
        }
      }
    }

    let changed = self.changed(&next_gen);
    self.cells = next_gen;
    changed
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // fn dump(room: &Room) {
  //   for (ix, cell) in room.cells.iter().enumerate() {
  //     match cell {
  //       Some(c) => {
  //         if c.occupied {
  //           print!("#")
  //         } else {
  //           print!("L")
  //         }
  //       }
  //       None => print!("."),
  //     }
  //     if (ix as i32) % room.width == room.width - 1 {
  //       println!("");
  //     }
  //   }
  // }

  // #[test]
  // fn test_dump() {
  //   let data = ["LL.", "L..", "..."];
  //   let room = Room::from(&data);

  //   println!("");
  //   dump(&room);
  // }

  fn get_test_data() -> &'static [&'static str] {
    &[
      "L.LL.LL.LL",
      "LLLLLLL.LL",
      "L.L.L..L..",
      "LLLL.LL.LL",
      "L.LL.LL.LL",
      "L.LLLLL.LL",
      "..L.L.....",
      "LLLLLLLLLL",
      "L.LLLLLL.L",
      "L.LLLLL.LL",
    ]
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1(get_test_data()), 37);
  }

  #[test]
  fn test_part_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0);
  }

  #[test]
  fn test_room_count_neighbors_of() {
    let data = ["LLL", "LLL", "LLL"];

    let mut room = Room::from(&data);
    for ix in [0, 1, 3].iter() {
      match room.cells.get_mut(*ix as usize) {
        Some(Some(cell)) => cell.occupied = true,
        _ => {}
      }
    }

    assert!(room.cells.get(0).is_some());
    assert!(room.cells.get(0).unwrap().is_some());
    assert_eq!(room.cells.get(0).unwrap().unwrap().occupied, true);

    assert_eq!(room.count_neighbors_of(room.get(0, 0).unwrap()), 2);
    assert_eq!(room.count_neighbors_of(room.get(0, 1).unwrap()), 2);
    assert_eq!(room.count_neighbors_of(room.get(0, 2).unwrap()), 1);
    assert_eq!(room.count_neighbors_of(room.get(1, 0).unwrap()), 2);
    assert_eq!(room.count_neighbors_of(room.get(1, 1).unwrap()), 3);
    assert_eq!(room.count_neighbors_of(room.get(1, 2).unwrap()), 1);
    assert_eq!(room.count_neighbors_of(room.get(2, 0).unwrap()), 1);
    assert_eq!(room.count_neighbors_of(room.get(2, 1).unwrap()), 1);
    assert_eq!(room.count_neighbors_of(room.get(2, 2).unwrap()), 0);
  }

  #[test]
  fn test_room_from() {
    let room = Room::from(get_test_data());

    assert!(room.get(-1, 0).is_none());
    assert!(room.get(0, -1).is_none());
    assert!(room.get(10, 0).is_none());
    assert!(room.get(0, 10).is_none());

    assert!(room.get(0, 0).is_some());
    assert_eq!(room.get(0, 0).unwrap().occupied, false);

    assert!(room.get(3, 6).is_none());
  }
}
