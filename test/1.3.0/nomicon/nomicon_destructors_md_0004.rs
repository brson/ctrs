fn main() {
    enum Link {
        Next(Box<Link>),
        None,
    }
}
