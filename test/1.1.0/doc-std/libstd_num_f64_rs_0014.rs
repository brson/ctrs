fn main() {
    use std::f64;
    
    let nan = f64::NAN;
    
    let f = 7.0_f64;
    let g = -7.0_f64;
    
    assert!(!f.is_sign_negative());
    assert!(g.is_sign_negative());
    // Requires both tests to determine if is `NaN`.
    assert!(!nan.is_sign_positive() && !nan.is_sign_negative());
}
