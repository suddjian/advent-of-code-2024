use std::{collections::HashMap, io::stdin};

// maps of pages that should come after the given page, if present.
fn read_rules(stream: &mut impl Iterator<Item = String>) -> HashMap<i32, Vec<i32>> {
  stream.by_ref()
    .map_while(|line|
      if line.len() > 0 {
        let mut pages =
          line.split("|").map(|x| x.parse().unwrap());
        Some((pages.next().unwrap(), pages.next().unwrap()))
      } else {
        None // first blank line indicates end of rules
      }
    )
    .fold(HashMap::default(), |mut acc, rule| {
      acc.entry(rule.0)
        .or_insert_with(Vec::new)
        .push(rule.1);
      return acc
    })
}

fn follows_rules(rules: &HashMap<i32, Vec<i32>>, pages: &Vec<i32>) -> bool {
  for (i, page) in pages.iter().enumerate() {
    match rules.get(page) {
      None => { continue }
      Some(applicable_rules) => {
        // must either have each "later" page later, or not at all.
        // So, actually, we just need to make sure that the j *isn't before* i
        for rule in applicable_rules {
          let pos = pages[0..i].iter().position(|x| x == rule);
          if pos.is_some() {
            return false
          }
        }
      }
    }
  }
  return true
}

pub fn part1() {
  let mut input =
    stdin().lines().map(|line| line.unwrap());

  let rules = read_rules(&mut input);

  let result = input.map(|line| {
    line.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>()
  })
  .filter(|update| follows_rules(&rules, update))
  .map(|update| update[update.len() / 2])//.collect::<Vec<i32>>();
  .reduce(|a, b| a + b).unwrap();

  println!("{result}");
}