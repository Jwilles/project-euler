fn is_palindrome(prod: u32) -> bool {
  let num_string = prod.to_string();
  let num_length = num_string.len();

  num_string.bytes().take(num_length/2).eq(num_string.bytes().rev().take(num_length/2))
}

fn main() {
  
  let mut largest_palindrome = 0;

  for i in 100..1000 {
    for j in 100..1000 {
      let mut prod = i * j;
      
      if largest_palindrome < prod && is_palindrome(prod) {
        largest_palindrome = prod;
      }
    }
  }

  println!("{}", largest_palindrome);
}
