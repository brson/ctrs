use std::ops::Shl;

#[derive(Copy, Clone)]
struct Foo;

impl Shl<Foo> for Foo {
    type Output = Foo;

    fn shl(self, _rhs: Foo) -> Foo {
        println!("Shifting left!");
        self
    }
}

fn main() {
    Foo << Foo;
}
