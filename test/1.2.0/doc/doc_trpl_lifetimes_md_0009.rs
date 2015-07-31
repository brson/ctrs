fn main() {
    fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
       x
    }
}
