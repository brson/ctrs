use std::ops::Rem;

#[derive(Copy, Clone)]
struct Foo;

impl Rem for Foo {
type Output = Foo;

fn rem(self, _rhs: Foo) -> Foo {
println!("Remainder-ing!");
self
}
}

fn main() {
Foo % Foo;
}
