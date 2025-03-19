// constant
const PI: f64 = 3.14159; 

fn main() {
    // static variable
    static MAX_VALUE: i32 = 100;
    
    // explicit
    let foo: &str = "foo";
    
    // type inferred
    let bar = "bar";
    
    // mutable variable
    let mut qux = "qux";
    qux = "qux!";
    
    // shadowing
    let bar = 5;
    
    println!("{} {} {}", foo, bar, qux);
    println!("{}", PI);
    println!("{}", MAX_VALUE);
}
