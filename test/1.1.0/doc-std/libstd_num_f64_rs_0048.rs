fn main() {
    use std::f64;
    
    let e = f64::consts::E;
    let x = 1.0_f64;
    
    let f = x.tanh();
    // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    let g = (1.0 - e.powi(-2))/(1.0 + e.powi(-2));
    let abs_difference = (f - g).abs();
    
    assert!(abs_difference < 1.0e-10);
}
