use std::io::{stdin, BufRead, BufReader};

// struct Grid<T> {
//   grid: Vec<Vec<T>>,
//   width: usize,
//   height: usize,
// }

fn read_grid(stream: impl BufRead) -> Vec<Vec<char>> {
  return stream.lines()
    .map(|line: Result<String, std::io::Error>|
      line.unwrap().chars().collect::<Vec<char>>()
    )
    .filter(|row| row.len() > 0)
    .collect();
}

const SEARCH: [char; 4] = ['X','M','A','S'];

fn map_search(x: usize, y: usize, dx: i32, dy: i32) -> Vec<(usize, usize)> {
  (0..SEARCH.len())
    .map(|i| (
      (x as i32 + dx * i as i32) as usize,
      (y as i32 + dy * i as i32) as usize
    ))
    .collect()
}

fn print_debug_grid(grid: Vec<Vec<char>>) {
  let printable_debug_grid = grid.iter().map(|row| row.iter().cloned().collect()).collect::<Vec<String>>().join("\n");
  println!("{printable_debug_grid}");
}

pub fn part1() {
  let grid = read_grid(BufReader::new(stdin()));

  let width = grid[0].len();
  let height = grid.len();
  let mut count = 0;

  let mut debug_grid = vec![vec!['.'; width]; height];

  for y in 0..height {
    for x in 0..width {
      let root = grid[y][x];
      if root != 'X' {
        continue
      }
      // search left, left-up, up, up-right, ...
      for dy in -1i32..=1 {
        for dx in -1i32..=1 {
          let yb = y as i32 + dy * 3;
          let xb = x as i32 + dx * 3;
          if yb >= height as i32 || yb < 0 || xb >= width as i32 || xb < 0 {
            continue
          }

          let mut good = true;
          let test = map_search(x, y, dx, dy);
          for (i, (x2, y2)) in test.iter().enumerate() {
            if grid[*y2][*x2] != SEARCH[i as usize] {
              good = false;
              break
            }
          }
          if good {
            count += 1;
            for (x2, y2) in test.iter() {
              debug_grid[*y2][*x2] = grid[*y2][*x2];
            }
          }
        }
      }
    }
  }

  println!("count: {count}");
  print_debug_grid(debug_grid);
}

fn crossmastest(a: char, b: char) -> bool {
  return a == 'M' && b == 'S' || a == 'S' && b == 'M'
}

pub fn part2() {
  let grid = read_grid(BufReader::new(stdin()));

  let width = grid[0].len();
  let height = grid.len();
  let mut count = 0;

  let mut debug_grid = vec![vec!['.'; width]; height];

  for y in 1..height-1 {
    for x in 1..width-1 {
      let root = grid[y][x];
      if root != 'A' {
        continue
      }

      if crossmastest(grid[y-1][x-1], grid[y+1][x+1]) && crossmastest(grid[y+1][x-1], grid[y-1][x+1]) {
        count += 1;
        debug_grid[y][x] = grid[y][x];
        debug_grid[y-1][x-1] = grid[y-1][x-1];
        debug_grid[y+1][x+1] = grid[y+1][x+1];
        debug_grid[y+1][x-1] = grid[y+1][x-1];
        debug_grid[y-1][x+1] = grid[y-1][x+1];
      }
    }
  }

  print_debug_grid(debug_grid);
  println!("count: {count}");
}