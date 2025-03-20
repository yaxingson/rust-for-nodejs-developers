![](./logo.png)

# Rust for Node.js Developers

> Examples of [Rust](https://www.rust-lang.org/) examples compared to [Node.js](https://nodejs.org) for learning.

This guide full of examples is intended for people learning Rust that are coming from Node.js, although the vice versa can work too. This is not meant to be a complete guide and it is assumed that you've gone through the [Getting Started of Rust]() tutorial. This guide is meant to be barely good enough to help you at a 
high level understand how to do X in Y and doing further learning on your own is of course required.

## Examples

### comments

Nodejs

```js
// this is a line comment

/*

this is a block comment

*/

```

Rust

```rs
fn main() {
  // this is a line comment 

  /*

  this is a block comment

  */

}

```

### printing

Nodejs

```js
console.log('print to stdout')
console.log('format %s %d', 'example', 1)
console.error('print to stderr')

```

Rust

```rs
fn main() {
  println!("print to stdout");
  println!("format {} {}", "example", 1);
  eprintln!("print to stderr");
  
}

```

### logging

Nodejs

```js
console.log((new Date()).toISOString(), 'hello world')

```

Rust

```rs


```

### variables

Nodejs

```js
// function scoped
var foo = 'foo'

// block scoped
let bar = 'bar'

// constant
const qux = 'qux'

```

Rust

```rs
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

```

### types

Nodejs

```js
// primitives
const myBool = true
const myNumber = 10
const myString = 'foo'
const mySymbol = Symbol('bar')
const myNull = null
const myUndefined = undefined

// object types
const myObject = {}
const myArray = []
const myFunction = function() {}
const myError = new Error('error')
const myDate = new Date()
const myRegex = /a/
const myMap = new Map()
const mySet = new Set()
const myPromise = Promise.resolve()
const myGenerator = function *() {}
const myClass = class {}

```

Rust

```rs
// scalar types
const myBool: bool = true;
const myInt8: i8 = 10;
const myUint8: u8 = 10;
const myChar: char = '😻';
const myFloat32: f32 = 3.0;
const myFloat64: f64 = 1.0;

// compound types
const myTuple: (i32, f64, u8) = (500, 6.4, 1);
const myArray: [i32; 5] = [1, 2, 3, 4, 5];

// standard library types
const myStr: &str = "foo";
const myString: String = String::from("Hello");

```

### type check

Nodejs

```js
function typeOf(obj) {
  return {}.toString.call(obj).split(' ')[1].slice(0,-1).toLowerCase()
}

const values = [
  true,
  10,
  'foo',
  Symbol('bar'),
  null,
  undefined,
  NaN,
  {},
  [],
  function(){},
  new Error(),
  new Date(),
  /a/,
  new Map(),
  new Set(),
  Promise.resolve(),
  function *() {},
  class {},
]

for (value of values) {
  console.log(typeOf(value))
}

```

Rust

```rs


```

### interpolation

Nodejs

```js
const name = 'bob'
const age = 21
const message = `${name} is ${age} years old`

console.log(message)

```
Rust

```rs
fn main() {
  let name = "bob";
  let age = 21;
  let message = format!("{} is {} years old", name, age);
  
  println!("{}", message);
}

```

### if/else

Nodejs

```js
const array = [1, 2]

if (array) {
  console.log('array exists')
}

if (array.length === 2) {
  console.log('length is 2')
} else if (array.length === 1) {
  console.log('length is 1')
} else {
  console.log('length is other')
}

const isOddLength = array.length % 2 == 1 ? 'yes' : 'no'

console.log(isOddLength)

```

Rust

```rs
fn main() {
  let arr = [1, 2];
  
  if arr.len() == 2 {
    println!("length is 2");
  } else if arr.len() == 1 {
    println!("length is 1");
  } else {
    println!("length is other");  
  }
  
  let is_odd_length = if arr.len() % 2 == 1 { "yes" } else { "no" };
  
  println!("{:?} {}", arr, is_odd_length);
}

```

### for

Nodejs

```js
for (let i = 0; i <= 5; i++) {
  console.log(i)
}

```
Rust

```rs
fn main() {
  for i in 0..6 {
    println!("{}", i);
  }
}

```

### while

Nodejs

```js
let i = 0

while (i <= 5) {
  console.log(i)
  i++
}

```

Rust

```rs
fn main() {
  let mut i = 0;
  
  while i <= 5 {
    println!("{}", i);
    i += 1;  
  }
}

```

### switch

Nodejs

```js
const value = 'b'

switch(value) {
  case 'a':
    console.log('A')
    break
  case 'b':
    console.log('B')
    break
  case 'c':
    console.log('C')
    break
  default:
    console.log('first default')
}

switch(value) {
  case 'a':
    console.log('A - falling through')
  case 'b':
    console.log('B - falling through')
  case 'c':
    console.log('C - falling through')
  default:
    console.log('second default')
}

```
Rust

```rs
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

```

### arrays

### uint8 arrays

### array iteration

### buffers

### maps

### objects

### functions

### default values

### destructuring

### spread operator

### rest operator

### swapping

### classes

### generators

### datetime

### timeout

### interval

### iife

### files

### json

### big numbers

### promises

### async/await

### streams

### event emitter

### errors

### try/catch

### exceptions

### regex

### exec(sync/async)

### tcp server

### udp server

### http server

### url parse

### gzip

### dns

### crypto

### env vars

### cli args

### cli flags

### stdout

### stderr

### stdin

### modules

### stack trace

### databases

### testing

### benchmark

### documentation

## Contributing

## License

Released under the [MIT](./LICENSE) license. ©[Yaxing Son](https://github.com/yaxingson)
