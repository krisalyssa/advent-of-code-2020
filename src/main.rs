use std::env;

mod day_01;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("usage: {} <day>", args[0]);
    std::process::exit(1);
  }

  let mut day = args[1].to_string();
  if day.len() < 2 {
    day = String::from("0".to_string() + &day);
  };

  let result = match day.as_str() {
    "01" => day_01::run(),
    _ => {
      eprintln!("unrecognized day '{}'", day);
      std::process::exit(1);
    }
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
