fn main() {
    fn bar<'a>() {
        let s: &'static str = "hi";
        let t: &'a str = s;
    }
}
