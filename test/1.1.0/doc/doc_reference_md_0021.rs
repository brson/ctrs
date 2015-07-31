fn main() {
    fn my_err(s: &str) -> ! {
        println!("{}", s);
        panic!();
    }
}
