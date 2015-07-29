fn main() {
    trait Printable {
      fn make_string(&self) -> String;
    }
    
    impl Printable for String {
        fn make_string(&self) -> String {
            (*self).clone()
        }
    }
}
