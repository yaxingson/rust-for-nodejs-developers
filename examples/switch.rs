fn main() {
  let value = 'b';
  
  match value {
    'a' => println!("A"),
    'b' => println!("B"),
    'c' => println!("C"),
    _ => println!("first default")
  }
  
  match value {
    'a' => println!("A"),
    'b' => {
        println!("B");
        println!("C");
    }
    _ => println!("first default")
  }
}
