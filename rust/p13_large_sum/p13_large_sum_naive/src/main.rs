use std::io::{BufRead, BufReader};
use std::fs::File;

extern crate num;
use num::bigint::BigInt;
use num::bigint::Sign;


fn read_numbers(filename: &str) -> Vec<BigInt> {

  let mut numbers = Vec::new();

  let file = File::open(filename).expect("cannot open file");
  let file = BufReader::new(file);

//  println!("{:?}", Sign::Plus);

  for line in file.lines().filter_map(|result| result.ok()) {

    let chars = line.chars();
    let mut res = Vec::new();
    
    for char in chars {
     res.push(char.to_string().parse().unwrap());
    }
     
//    println!("{}", line);
//    let res: Vec<u32> = line.split(' ').map(|s| s.parse().unwrap()).collect();
 //   println!("{:?}", res);
//    numbers.push(res);    
    numbers.push(BigInt::new(Sign::Plus, res));
  }

  return numbers;
}


fn main() {

  let filename = "numbers.txt";
  let numbers = read_numbers(filename);

  let mut sum_total = BigInt::new(Sign::Plus, vec![0]); 

  for number in numbers {
    sum_total = sum_total + number;   
    println!("{:?}", sum_total);
  // sum_total = sum_total::Add(number); 
  }


//  println!("{:?}", numbers);
//  println!("{}", sum_total::to_u64);

}

