trait Foo {
    fn foo(&self);
    fn bar(&self);
}
struct MyStruct;

impl Foo for MyStruct {
    fn foo(&self) {
        // implementation goes here
    }

    fn bar(&self) {
        // let's not worry about implementing bar() for now
        unimplemented!();
    }
}

fn main() {
    let s = MyStruct;
    s.foo();

    // we aren't even using bar() yet, so this is fine.
}
