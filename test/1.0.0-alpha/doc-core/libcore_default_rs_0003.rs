use std::default::Default;
#[derive(Default)]
struct SomeOptions {
    foo: int,
    bar: f32,
}
fn main() {
    let options = SomeOptions { foo: 42, ..Default::default() };
}
