fn main() {
    fn diverges() -> ! {
        panic!("This function never returns!");
    }
}
