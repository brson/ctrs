fn main() {
    use std::collections::VecDeque;        let vec = vec![1, 2, 3, 4];    let buf: VecDeque<_> = vec.into_iter().collect();}
