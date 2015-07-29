fn main() {
    struct X;
    impl X { fn iter<T, F>(&self, _: F) where F: FnMut() -> T {} } let b = X;
    b.iter(|| {
        // note lack of `;` (could also use an explicit `return`).
        (0..1000).fold(0, |old, new| old ^ new)
    });
}
