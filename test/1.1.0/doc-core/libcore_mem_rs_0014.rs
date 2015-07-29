fn main() {
    use std::mem;    struct Buffer<T> { buf: Vec<T> }    impl<T> Buffer<T> {        fn get_and_reset(&mut self) -> Vec<T> {            mem::replace(&mut self.buf, Vec::new())        }    }}
