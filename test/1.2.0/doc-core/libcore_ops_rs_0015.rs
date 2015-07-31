use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
struct Foo;
struct Bar;

impl Index<Bar> for Foo {
type Output = Foo;

fn index<'a>(&'a self, _index: Bar) -> &'a Foo {
self
}
}

impl IndexMut<Bar> for Foo {
fn index_mut<'a>(&'a mut self, _index: Bar) -> &'a mut Foo {
println!("Indexing!");
self
}
}

fn main() {
&mut Foo[Bar];
}
