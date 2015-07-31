fn main() {
    let x = 1.65f32;
    assert!(x == x.trunc() + x.fract())
}
