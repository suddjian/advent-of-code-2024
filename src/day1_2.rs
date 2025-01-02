use std::{collections::HashMap, error, io::{stdin, BufRead, BufReader}};

fn read_2_i32_cols<R: BufRead>(stream: R) -> Result<(Vec<i32>, Vec<i32>), Box<dyn error::Error>> {
  let mut list_a = Vec::new();
  let mut list_b = Vec::new();

  for line in stream.lines() {
    let content = line?;
    if let Some((a, b)) = content.split_once("   ") {
      list_a.push(a.parse()?);
      list_b.push(b.parse()?);
    }
  }
  return Ok((list_a, list_b));
}

fn main() {
  let (a, b) = read_2_i32_cols(BufReader::new(stdin())).unwrap();

  let mut multipliers = HashMap::new();
  for x in b {
    let multiplier = multipliers.entry(x).or_insert(0);
    *multiplier += 1;
  }
  
  let result = a.iter()
    .map(|x| x * *multipliers.entry(*x).or_insert(0))
    .reduce(|sum, x| sum + x)
    .unwrap_or(0);

  println!("{result}");
}
