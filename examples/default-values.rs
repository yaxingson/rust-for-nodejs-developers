fn greet(name:Option<String>) -> String {
  let name = name.unwrap_or("stranger".to_string());
  format!("hello {}", name)
}

fn main() {
  println!("{}", greet(None));
  println!("{}", greet(Some("bob".to_string())));
}
