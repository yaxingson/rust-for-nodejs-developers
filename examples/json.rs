use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Obj {
  foo: String
}

fn main() {
  let obj = Obj {
    foo: "bar".to_string()
  };

  let serialized = serde_json::to_string(&obj).unwrap();
  println!("Serialized JSON: {}", serialized);

  let deserialized: Obj = serde_json::from_str(&serialized).unwrap();
  println!("Deserialized person: {:?}", deserialized);
}
