use std::io::{stdin, Read};

use regex::Regex;

fn calculate(iter: impl Iterator<Item = (i32, i32)>) -> i32 {
  return iter
    .map(|(a, b)| a * b)
    .reduce(|a, b| a + b)
    .unwrap();
}

pub fn part1() {
  let mulre = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

  let mut corrupted = String::new();
  let _ = stdin().read_to_string(&mut corrupted);
  
  let result  = calculate(
    mulre.captures_iter(&corrupted)
    .map(|capture| {
      let (_, [a, b]) = capture.extract();
      (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
    })
  );
  
  println!("total: {result}");
}

pub fn part2() {
  let ctrlre = Regex::new(r"(?s)(?:^|don't\(\).*?do\(\))|don't\(\).*?$").unwrap();
  let mulre = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

  let mut corrupted = String::new();
  let _ = stdin().read_to_string(&mut corrupted);

  let total = calculate(
    ctrlre.split(&corrupted)
    .flat_map(|s| {dbg!(s); mulre.captures_iter(s)})
    .map(|capture| {
      let (_, [a, b]) = capture.extract();
      (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
    })
  );
  
  println!("total: {total}");
}