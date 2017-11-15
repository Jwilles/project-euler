fn is_prime(num: u64) -> bool {
  if num % 2 == 0 {
    return false;
  } else {
    let mut i = 3;
    while i < num {
      if num % i == 0 {
        return false;
      }
      i = i + 2;
    }
    return true;
  } 
}

fn main() {
  
  let mut summation = 0u64;

  for n in 2..2000001 {
    if is_prime(n) {
      summation = summation + n
    }  
  } 

  println!("{}", summation + 1)
}
