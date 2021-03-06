use std::ops::Mul;

#[derive(Copy, Clone)]
struct Foo;

impl Mul for Foo {
    type Output = Foo;

    fn mul(self, _rhs: Foo) -> Foo {
        println!("Multiplying!");
        self
    }
}

fn main() {
    Foo * Foo;
}
