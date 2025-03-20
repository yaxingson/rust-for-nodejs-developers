use std::collections::HashMap;

struct Obj {
  some_properties:HashMap<String, String>
}

impl Obj {
  fn new() -> Self {
    let mut some_properties = HashMap::new();

    some_properties.insert("foo".to_string(), "bar".to_string());
    
    Obj {
        some_properties
    }
  }

  fn some_method(&mut self, prop: &str) -> String {
    if let Some(val) = self.some_properties.get(prop) {
        return val.to_string();
    } else {
        return String::new();
    }
  }
}

fn main() {
  let mut person = Obj::new();
  
  println!("{:?}", person.some_properties);
  println!("{}", person.some_method("foo"));
}
