use std::ops::Shr;

#[derive(Copy, Clone)]
struct Foo;

impl Shr<Foo> for Foo {
type Output = Foo;

fn shr(self, _rhs: Foo) -> Foo {
println!("Shifting right!");
self
}
}

fn main() {
Foo >> Foo;
}
