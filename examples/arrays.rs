fn main() {
  let arr: [i8; 5] = [1, 2, 3, 4, 5];
  
  let clone = &arr[0..arr.len()];
  let sub = &arr[0..3];
  
  let concatenated = [clone, &[6, 7]].concat();
  let prepended = [&[-2, -1, 0], clone].concat();
  
  println!("{:?} {:?} {:?}", arr, clone, sub);
  println!("{:?}", concatenated);
  println!("{:?}", prepended);
}
