fn main() {
    let mut v = vec![1u, 2, 3, 4];
    unsafe {
        v.set_len(1);
    }
}
