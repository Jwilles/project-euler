use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;

fn build_grid(filename: &str) -> Vec<Vec<i32>> {

  let mut grid = Vec::new();

  let file = File::open(filename).expect("cannot open file");
  let file = BufReader::new(file);

  for line in file.lines().filter_map(|result| result.ok()) {
    let res: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
    grid.push(res);
  }

  return grid;
}


fn main() {

  let filename = "grid.txt";
  let grid = build_grid(filename);


  let rows = grid.len();
  let cols = grid[0].len();

  let mut largest_product = 0;

  for i in 0..20 {
    for j in 0..16 {
      if i < 16 {
        let right = grid[i][j] * grid[i][j+1] * grid[i][j+2] * grid[i][j+3];
        let up = grid[i][j] * grid[i+1][j] * grid[i+2][j] * grid[i+3][j];
        let diag = grid[i][j] * grid[i+1][j+1] * grid[i+2][j+2] * grid[i+3][j+3];
        largest_product = cmp::max(largest_product, vec![right, up, diag].iter().max().unwrap().abs());
      }
      if i >= 3 {
        let diag = grid[i][j] * grid[i-1][j+1] * grid[i-2][j+2] * grid[i-3][j+3];
        largest_product = cmp::max(largest_product, diag);
      }
    }  
  }
  
  println!("{:?}", grid);
  println!("{}", largest_product)
}




