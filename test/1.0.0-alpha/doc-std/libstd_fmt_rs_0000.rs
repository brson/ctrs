fn main() {
format!("Hello");                  // => "Hello"
format!("Hello, {}!", "world");    // => "Hello, world!"
format!("The number is {}", 1i);   // => "The number is 1"
format!("{:?}", (3i, 4i));         // => "(3i, 4i)"
format!("{value}", value=4i);      // => "4"
format!("{} {}", 1i, 2u);          // => "1 2"
}
