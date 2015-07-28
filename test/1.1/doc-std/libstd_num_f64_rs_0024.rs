fn main() {
    let two = 2.0_f64;
    
    // log2(2) - 1 == 0
    let abs_difference = (two.log2() - 1.0).abs();
    
    assert!(abs_difference < 1e-10);
}
