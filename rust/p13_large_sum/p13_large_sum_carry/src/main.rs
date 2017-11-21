use std::io::{BufRead, BufReader};
use std::fs::File;


fn read_numbers(filename: &str) -> Vec<Vec<u32>> {

  let mut numbers = Vec::new();

  let file = File::open(filename).expect("cannot open file");
  let file = BufReader::new(file);

  for line in file.lines().filter_map(|result| result.ok()) {
    let res: Vec<u32> = line.chars().map(|s| s.to_string().parse().unwrap()).collect();
    numbers.push(res);    
  }

  return numbers;
}


fn main() {

  let filename = "numbers.txt";
  let numbers = read_numbers(filename);

  let mut sum_total = Vec::new(); 
  let num_cols = numbers[0].len();
  let num_rows = numbers.len();
  let mut carry = 0;


  for i in (0..(num_cols)).rev() {

    let mut sum_col = carry;

    for j in 0..num_rows {
      sum_col = sum_col + numbers[j][i];      
    } 
    if i == 0 {
      sum_total.push(sum_col)
    } else {
      sum_total.push(sum_col % 10);
    }

    carry = sum_col / 10; 
}
  

  println!("{:?}", sum_total);
}

