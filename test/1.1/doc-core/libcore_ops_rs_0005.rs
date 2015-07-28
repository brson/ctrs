use std::ops::Div;

#[derive(Copy, Clone)]
struct Foo;

impl Div for Foo {
    type Output = Foo;

    fn div(self, _rhs: Foo) -> Foo {
        println!("Dividing!");
        self
    }
}

fn main() {
    Foo / Foo;
}
