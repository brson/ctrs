fn main() {
    trait Foo {
    fn clone(&self);
    }
    
    #[derive(Clone)]
    struct Bar;
    
    impl Foo for Bar {
    fn clone(&self) {
    println!("Making a clone of Bar");
    
    <Bar as Clone>::clone(self);
    }
    }
}
