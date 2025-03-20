struct Foo {
  item: String
}

impl Foo {
  fn new(value:String) -> Self {
    Foo {
        item:value
    }
  }

  fn set_item(&mut self, value:String) {
    self.item = value;
  }
  
  fn get_item(&self) -> String {
    self.item.clone()
  }
}

fn main() {
  let mut foo = Foo::new("bar".to_string());
  
  println!("{}", foo.item);
  
  foo.set_item("qux".to_string());
  
  println!("{}", foo.get_item());
}
