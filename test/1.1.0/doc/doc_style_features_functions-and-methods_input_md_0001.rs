fn main() {
    fn foo(b: &Bar) {
        let b = b.clone();
        // use b as owned after cloning
    }
}
