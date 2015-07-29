fn main() {
    let x = 1.0_f64;
    let f = x.sinh().asinh();
    
    let abs_difference = (f - x).abs();
    
    assert!(abs_difference < 1.0e-10);
}
