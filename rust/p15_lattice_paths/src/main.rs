fn main() {

  let mut paths: u64 = 1;

  for i in 1..21 {
    paths = paths * (40 + 1 - i) / i;
    println!("{}", paths);
  }
}
