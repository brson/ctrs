macro_rules! foo {
    () => (fn x() { });
}

fn main() {
    foo!();
    x();
}
