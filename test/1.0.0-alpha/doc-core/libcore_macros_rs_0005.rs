fn main() {
    fn foo(x: Option<int>) {
        match x {
            Some(n) if n >= 0 => println!("Some(Non-negative)"),
            Some(n) if n <  0 => println!("Some(Negative)"),
            Some(_)           => unreachable!(), // compile error if commented out
            None              => println!("None")
        }
    }
}
