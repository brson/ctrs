use std::ops::Sub;

#[derive(Copy, Clone)]
struct Foo;

impl Sub for Foo {
    type Output = Foo;

    fn sub(self, _rhs: Foo) -> Foo {
        println!("Subtracting!");
        self
    }
}

fn main() {
    Foo - Foo;
}
