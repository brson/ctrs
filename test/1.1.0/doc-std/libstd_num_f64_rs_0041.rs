fn main() {
    let f = 1.0_f64;
    
    // atan(tan(1))
    let abs_difference = (f.tan().atan() - 1.0).abs();
    
    assert!(abs_difference < 1e-10);
}
