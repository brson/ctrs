fn main() {
    use std::f64;
    
    let f = f64::consts::PI / 4.0;
    
    // acos(cos(pi/4))
    let abs_difference = (f.cos().acos() - f64::consts::PI / 4.0).abs();
    
    assert!(abs_difference < 1e-10);
}
