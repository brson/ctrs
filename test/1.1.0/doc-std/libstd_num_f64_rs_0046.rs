fn main() {
    use std::f64;
    
    let e = f64::consts::E;
    let x = 1.0_f64;
    
    let f = x.sinh();
    // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    let g = (e*e - 1.0)/(2.0*e);
    let abs_difference = (f - g).abs();
    
    assert!(abs_difference < 1e-10);
}
