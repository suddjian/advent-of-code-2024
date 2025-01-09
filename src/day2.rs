use std::{ io::stdin, str::FromStr};


fn parse_i32_list(list: &String) -> Result<Vec<i32>, <i32 as FromStr>::Err> {
  return list.split(" ")
    .map(|num| num.parse::<i32>())
    .collect();
}

fn is_report_safe(line: &String) -> bool {
  let report = parse_i32_list(line).unwrap();
  let is_increasing = report[1] > report[0];
  for i in 1..report.len() {
    let level = report[i];
    let prev = report[i-1];
    let diff = (level - prev).abs();
    if (level > prev) != is_increasing || diff > 3 || diff < 1 {
      return false;
    }
  }
  return true;
}

fn main() {
  let result =
    stdin().lines()
    .filter(|line| is_report_safe(line.as_ref().unwrap()))
    .count();
  println!("Safe: {result}");
}