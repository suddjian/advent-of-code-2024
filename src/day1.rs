use std::io::stdin;

fn main() {
  let stdin = stdin();

  let mut list_a: Vec<i32> = Vec::new();
  let mut list_b: Vec<i32> = Vec::new();

  for result in stdin.lines() {
    let line = result.unwrap();
    let content = line.split_once("   ").unwrap();
    let a = content.0.parse().unwrap();
    let b = content.1.parse().unwrap();
    list_a.push(a);
    list_b.push(b);
  }

  list_a.sort();
  list_b.sort();

  let mut total_distance = 0;
  for i in 0..list_a.len() {
    total_distance += (list_a[i] - list_b[i]).abs();
  }
  println!("total distance: {}", total_distance);
}
