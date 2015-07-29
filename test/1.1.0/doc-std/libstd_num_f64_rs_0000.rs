fn main() {
    use std::f64;
    
    let nan = f64::NAN;
    let f = 7.0_f64;
    
    assert!(nan.is_nan());
    assert!(!f.is_nan());
}
