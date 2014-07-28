fn main() {
    trait Printable { fn print(&self); }
    impl Printable for f32 {
        fn print(&self) { println!("{}", *self) }
    }
    
    impl Printable for bool {
        fn print(&self) { println!("{}", *self) }
    }
    
    true.print();
    3.14159.print();
}