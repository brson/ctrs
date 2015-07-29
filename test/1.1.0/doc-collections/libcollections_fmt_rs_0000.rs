fn main() {
    format!("Hello");                 // => "Hello"
    format!("Hello, {}!", "world");   // => "Hello, world!"
    format!("The number is {}", 1);   // => "The number is 1"
    format!("{:?}", (3, 4));          // => "(3, 4)"
    format!("{value}", value=4);      // => "4"
    format!("{} {}", 1, 2);           // => "1 2"
}
