fn main() {
    use std::f64;
    
    let x = 2.0*f64::consts::PI;
    
    let abs_difference = (x.cos() - 1.0).abs();
    
    assert!(abs_difference < 1e-10);
}
