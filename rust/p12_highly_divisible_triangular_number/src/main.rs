
fn gen_triangle(i: i32) -> i32 {
  return (i * (i+1)) / 2;
}

fn find_num_divisors(triangle: i32) -> i32 {

  let mut num_divisors = 0;
  

  for i in 1..((triangle as f64).sqrt() as i32) {
    if triangle % i == 0 {
      if triangle / i == i {
        num_divisors = num_divisors + 1;
      } else {
        num_divisors = num_divisors + 2;
      }
    }
  }
 
  return num_divisors;
}

fn main() {

  let mut max_num_divisors = 0;
  let mut i = 0;
  let mut triangle = 0;

  while max_num_divisors < 500 {
    triangle = gen_triangle(i);
    let num_divisors: i32 = find_num_divisors(triangle);    
    if num_divisors > max_num_divisors {
      max_num_divisors = num_divisors;
    }
    i = i + 1;
  }

  println!("{}", triangle);
  println!("{}", max_num_divisors);
}
