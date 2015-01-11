use std::default::Default;

enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Kind { Kind::A }
}

#[derive(Default)]
struct SomeOptions {
    foo: int,
    bar: f32,
    baz: Kind,
}


fn main() {
    let options: SomeOptions = Default::default();
}
