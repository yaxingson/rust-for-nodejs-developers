struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn get_position(&self) -> (i32, i32) {
    (self.x, self.y)
  }    
}

fn main() {
  let point = Point { x: 10, y: 20 };
  
  let Point { x, y } = point;
  let (m, n) = point.get_position();

  println!("x: {}, y: {}", x, y); 
  println!("m: {}, n: {}", m, n);
}
