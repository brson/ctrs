use std::ops::BitXor;

#[derive(Copy, Clone)]
struct Foo;

impl BitXor for Foo {
    type Output = Foo;

    fn bitxor(self, _rhs: Foo) -> Foo {
        println!("Bitwise Xor-ing!");
        self
    }
}

fn main() {
    Foo ^ Foo;
}
