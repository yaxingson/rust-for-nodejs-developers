fn main() {
  let arr = [1, 2];
  
  if arr.len() == 2 {
    println!("length is 2");
  } else if arr.len() == 1 {
    println!("length is 1");
  } else {
    println!("length is other");  
  }
  
  let is_odd_length = if arr.len() % 2 == 1 { "yes" } else { "no" };
  
  println!("{:?} {}", arr, is_odd_length);
}
