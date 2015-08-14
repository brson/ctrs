fn main() {
    fn get_box<'a>(str: &'a str) -> Box<&'a str> {
        // string literals are `&'static str`s
        Box::new("hello")
    }
}
