use std::env;

mod day3;
mod day4;
mod day5;

fn main() {
  let solutions: [Vec<fn()>; 3] = [
    vec![day3::part1 as fn(), day3::part2 as fn()],
    vec![day4::part1 as fn(), day4::part2 as fn()],
    vec![day5::part1 as fn()],
  ];

  let args: Vec<String> = env::args().collect();
  assert!(args.len() >= 2, "Puzzle name needed.");

  let day_num = args[1].as_str().parse::<usize>().unwrap();
  let puzzle_index = day_num - 3;
  assert!(puzzle_index < solutions.len(), "Unsolved puzzle day {day_num}");
  let puzzle = &solutions[puzzle_index];

  let part_num = match args.get(2) {
    Some(x) => x.parse::<usize>().unwrap(),
    None => 1,
  };
  let part_index = part_num - 1;
  assert!(part_index < puzzle.len(), "No part {part_index} for day {day_num}");
  puzzle[part_index]();
}
