use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {

  let file = File::open("grid.txt").expect("cannot open file");
  let file = BufReader::new(file);

  let mut grid = Vec::new();

  for line in file.lines().filter_map(|result| result.ok()) {
    let res: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
    grid.push(res);
  }

  println!("{:?}", grid);
}




