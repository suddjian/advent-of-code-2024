use std::env;

mod day3;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Puzzle name needed.");
  }

  let puzzle = args[1].as_str();
  let part = match args.get(2) {
    Some(x) => x,
    None => "1"
  };

 match (puzzle, part) {
    ("3", "1") => day3::part1(),
    ("3", "2") => day3::part2(),
    _ => panic!("Unknown puzzle day {puzzle} part {part}"),
  }
}
