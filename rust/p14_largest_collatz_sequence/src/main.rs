

fn main() {

  let mut max_count = 0;
  let mut max_num = 0;

  for i in 1..1000001 {
    let mut n: u64 = i;
    let mut count = 0;
    while n != 1 {
      count = count + 1;
      if n % 2 == 0 {
        n = n / 2; 
      } else {
        n = (3 * n) + 1;   
      }
    }
    if max_count < count {
      max_count = count;
      max_num = i;
    }
  }
  println!("{}", max_count);
  println!("{}", max_num);

}
