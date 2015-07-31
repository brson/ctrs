fn main() {
    format!("{argument}", argument = "test");   // => "test"
    format!("{name} {}", 1, name = 2);        // => "2 1"
    format!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b"
}
