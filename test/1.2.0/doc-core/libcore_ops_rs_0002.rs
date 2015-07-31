use std::ops::Add;

#[derive(Copy, Clone)]
struct Foo;

impl Add for Foo {
type Output = Foo;

fn add(self, _rhs: Foo) -> Foo {
println!("Adding!");
self
}
}

fn main() {
Foo + Foo;
}
