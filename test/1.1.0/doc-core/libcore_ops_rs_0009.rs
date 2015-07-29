use std::ops::BitAnd;

#[derive(Copy, Clone)]
struct Foo;

impl BitAnd for Foo {
    type Output = Foo;

    fn bitand(self, _rhs: Foo) -> Foo {
        println!("Bitwise And-ing!");
        self
    }
}

fn main() {
    Foo & Foo;
}
