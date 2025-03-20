fn main() {
  let mut a = "foo";
  let mut b = "bar";
  
  println!("{} {}", a, b);
  
  (b, a) = (a, b);
  
  println!("{} {}", a, b);
}
