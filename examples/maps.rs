use std::collections::HashMap;

fn main() {
  let mut map: HashMap<String, &str> = HashMap::new();
  
  map.insert("foo".to_string(), "bar");
  
  let found = map.contains_key("foo");
  
  map.remove("foo");
  
  if let Some(item) = map.get("foo") {
      println!("item = {}", item);
  } else {
      println!("key not exist");
  }
  
  println!("{:?} {}", map, found);
}
