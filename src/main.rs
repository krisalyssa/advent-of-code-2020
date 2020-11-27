mod day_01;

pub enum Day {
  _01,
}

fn main() {
  let day = Day::_01;

  let result = match day {
    Day::_01 => day_01::run(),
  };

  if let Some((time_part_1, time_part_2, time_total)) = result {
    println!(
      "Time: part 1 = {} µs, part 2 = {} µs, total = {} µs",
      time_part_1.as_micros(),
      time_part_2.as_micros(),
      time_total.as_micros()
    );
  }
}
