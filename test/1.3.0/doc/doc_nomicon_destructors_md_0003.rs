fn main() {
    struct Boxy<T> {
        data1: Box<T>,
        data2: Box<T>,
        info: u32,
    }
}
