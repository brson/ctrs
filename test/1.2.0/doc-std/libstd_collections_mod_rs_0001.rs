fn main() {
    let mut vec = vec![1, 2, 3, 4];
    for x in vec.iter_mut() {
    *x += 1;
    }
}
