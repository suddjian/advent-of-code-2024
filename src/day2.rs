use std::{ cmp::max, io::stdin, str::FromStr};

fn parse_i32_list(list: String) -> Result<Vec<i32>, <i32 as FromStr>::Err> {
  return list.split(" ")
    .map(|num| num.parse::<i32>())
    .collect();
}

fn is_report_safe(report: &Vec<i32>) -> bool {
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

fn find_dominant_sign(report: &Vec<i32>) -> i32 {
  let mut sum = 0;
  for i in 1..4 {
    sum += (report[i] - report[i-1]).signum();
  }
  return sum.signum();
}

#[derive(PartialEq)]
enum Slope {
  Ok,
  WrongWay,
  TooSteep,
}

fn check_slope(a: i32, b: i32, sign: i32) -> Slope {
  let diff = b - a;
  if diff.signum() != sign {
    return Slope::WrongWay;
  } else if diff.abs() > 3 {
    return Slope::TooSteep;
  } else {
    return Slope::Ok;
  }
}

fn without<T: Clone>(vec: &Vec<T>, index: usize) -> Vec<T> {
  let mut result = Vec::with_capacity(max(0, vec.len()-2));
  result.extend_from_slice(&vec[0..index]);
  if index < vec.len() - 1 {
    result.extend_from_slice(&vec[index+1..vec.len()]);
  }
  return result;
}

fn is_report_safe_with_problem_dampener(report: &Vec<i32>) -> bool {
  let sign = find_dominant_sign(&report);
  let len = report.len();

  for i in 1..len {
    let slope = check_slope(report[i-1], report[i], sign);
    if slope != Slope::Ok {
      return 
        i == len-1
        || is_report_safe(&without(report, i-1))
        || is_report_safe(&without(report, i));
    }
  }
  return true;
}

fn main() {
  let result =
    stdin().lines()
    .map(|line| line.unwrap())
    .map(parse_i32_list)
    .map(|report| report.unwrap())
    .filter(|report| is_report_safe_with_problem_dampener(report))
    .count();
  println!("Safe: {result}");
}