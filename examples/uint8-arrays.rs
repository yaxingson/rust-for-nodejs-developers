fn main() {
  let mut arr = [0u8; 10];
  
  let sub = &arr[2..];
  let sub2 = &arr[2..4];
  
  println!("{:?} {}", arr, arr.len());
  println!("{:?} {:?}", sub, sub2);
  
  (&mut arr[5..10]).fill(9);
  
  println!("{:?}", arr);
}
