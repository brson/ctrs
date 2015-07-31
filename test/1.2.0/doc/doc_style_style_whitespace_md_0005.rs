fn main() {
    // Good
    struct Foo {
    short: f64,
    really_long: f64,
    }
    
    // Bad
    struct Bar {
    short:       f64,
    really_long: f64,
    }
    
    // Good
    let a = 0;
    let radius = 7;
    
    // Bad
    let b        = 0;
    let diameter = 7;
}
