fn main() {
    let mut vec1 = vec![1, 2, 3, 4];    let vec2 = vec![10, 20, 30, 40];    vec1.extend(vec2.into_iter());}
