fn main() {
let mut x = vec!["Hello", "world"];

{
let y = &x[0];
}

x.push("foo");
}
