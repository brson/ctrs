fn main() {
    let mut s = String::from("hello");
    unsafe {
    let vec = s.as_mut_vec();
    assert!(vec == &[104, 101, 108, 108, 111]);
    vec.reverse();
    }
    assert_eq!(s, "olleh");
}
