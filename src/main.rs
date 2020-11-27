mod day_01;

pub enum Day {
  _01,
}

fn main() {
  let day = Day::_01;
  match day {
    Day::_01 => day_01::run(),
  }
}
