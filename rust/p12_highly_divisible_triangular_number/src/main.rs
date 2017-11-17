
fn gen_triangle() {

}

fn find_num_divisors() {

}

fn main() {

  let mut max_num_divisors = 0;
  let mut i = 0;

  while num_divisors < 500 {
    let triangle = gen_triangle(i);
    let num_divisors: i32 = find_num_divisors(triangle);    
    i = i + 1
  }
}
