fn main() {
  let arr = ["a", "b", "c"];
  
  arr.iter().for_each(|num| println!("{}", num));
  
  let mapped:Vec<_> = arr.iter().map(|x| x.to_uppercase()).collect();
  let filtered: Vec<_> = arr.iter().filter(|x| x.starts_with("b")).collect();
  let reduced = arr.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));
  
  println!("{:?}", mapped);
  println!("{:?}", filtered);
  println!("{:?}", reduced);
}
