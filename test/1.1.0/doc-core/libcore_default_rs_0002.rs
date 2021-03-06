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
    foo: i32,
    bar: f32,
    baz: Kind,
}


fn main() {
    let options: SomeOptions = Default::default();
}
