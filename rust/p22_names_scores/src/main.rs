use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_names(filename: &str) -> Vec<&str> {

  let file = File::open(filename).expect("cannot open file");
  let file = BufReader::new(file);

  let mut vec = Vec::new();

  for line in file.lines().filter_map(|result| result.ok()) {
    let mut res = line.split(",");
    vec = res.collect::<Vec<&str>>();
  }

  return vec;
}

fn main() {
  let filename = "names.txt";
  let names = read_names(filename);

  println!("{:?}", names);
}
