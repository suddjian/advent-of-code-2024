use std::io::stdin;


fn read_rules(stream: &mut impl Iterator<Item = String>) -> Vec<(i32, i32)> {
  return stream.by_ref().map_while(|line|
    if line.len() > 0 {
      let mut nums =
        line.split("|").map(|x| x.parse().unwrap());
      Some((nums.next().unwrap(), nums.next().unwrap()))
    } else {
      None
    }
  ).collect()
}

pub fn part1() {
  let mut input =
    stdin().lines().map(|line| line.unwrap());
  let rules = read_rules(&mut input);
  let result = input.map(|line| {
    line.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>()
  })
  .filter(|update| {
    for rule in &rules {
      let mut first = false;
      let mut second = false;
      'pages: for page in update {
        if *page == rule.0 {
          first = true;
          if second {
            return false
          }
        } else if *page == rule.1 {
          second = true;
          if first {
            break 'pages;
          }
        }
      }
    }
    return true;
  })
  .map(|update| update[update.len() / 2])//.collect::<Vec<i32>>();
  .reduce(|a, b| a + b).unwrap();
  dbg!(result);
}