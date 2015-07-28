fn main() {
    use std::f64;
    
    let f = f64::consts::PI / 2.0;
    
    // asin(sin(pi/2))
    let abs_difference = (f.sin().asin() - f64::consts::PI / 2.0).abs();
    
    assert!(abs_difference < 1e-10);
}
