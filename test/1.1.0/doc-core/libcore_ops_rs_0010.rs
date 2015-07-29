use std::ops::BitOr;

#[derive(Copy, Clone)]
struct Foo;

impl BitOr for Foo {
    type Output = Foo;

    fn bitor(self, _rhs: Foo) -> Foo {
        println!("Bitwise Or-ing!");
        self
    }
}

fn main() {
    Foo | Foo;
}
