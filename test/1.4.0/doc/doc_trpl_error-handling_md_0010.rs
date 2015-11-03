fn main() {
    fn unwrap_or<T>(option: Option<T>, default: T) -> T {
        match option {
            None => default,
            Some(value) => value,
        }
    }
}
