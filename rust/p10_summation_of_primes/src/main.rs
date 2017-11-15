fn is_prime(num: u64) -> bool {

  if num <= 1 {
    return false;
  }

  if num <= 3 {
    return true;
  }

  if num % 2 == 0 || num % 3 == 0 {
    return false;
  }

  let mut i = 5;
  while i*i <= num {
    if num % i == 0 || num % (i+2) == 0 {
      return false;
    }
    i = i + 6;
  } 
  
  return true;
}

fn main() {
  
  let mut summation = 0u64;

  for n in 0..2000001 {
    if is_prime(n) {
      summation = summation + n
    }  
  } 

  println!("{}", summation)
}
