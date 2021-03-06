use std::ops::Index;

#[derive(Copy, Clone)]
struct Foo;
struct Bar;

impl Index<Bar> for Foo {
    type Output = Foo;

    fn index<'a>(&'a self, _index: Bar) -> &'a Foo {
        println!("Indexing!");
        self
    }
}

fn main() {
    Foo[Bar];
}
