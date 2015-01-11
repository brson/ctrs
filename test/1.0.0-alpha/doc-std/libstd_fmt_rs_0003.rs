fn main() {
format!("{argument}", argument = "test");   // => "test"
format!("{name} {}", 1i, name = 2i);        // => "2 1"
format!("{a} {c} {b}", a="a", b='b', c=3i);  // => "a 3 b"
}
