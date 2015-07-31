use std::ops::Not;

#[derive(Copy, Clone)]
struct Foo;

impl Not for Foo {
type Output = Foo;

fn not(self) -> Foo {
println!("Not-ing!");
self
}
}

fn main() {
!Foo;
}
