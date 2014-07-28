fn main() {
    fn line(a: int, b: int, x: int) -> int { a * x + b  }
    fn oops(a: int, b: int, x: int) -> ()  { a * x + b; }
    
    assert!(8 == line(5, 3, 1));
    assert!(() == oops(5, 3, 1));
}