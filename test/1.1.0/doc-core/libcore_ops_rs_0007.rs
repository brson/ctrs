use std::ops::Neg;

#[derive(Copy, Clone)]
struct Foo;

impl Neg for Foo {
    type Output = Foo;

    fn neg(self) -> Foo {
        println!("Negating!");
        self
    }
}

fn main() {
    -Foo;
}
