fn main() {
    use std::f32;
    
    let nan = f32::NAN;
    let f = 7.0_f32;
    let g = -7.0_f32;
    
    assert!(f.is_sign_positive());
    assert!(!g.is_sign_positive());
    // Requires both tests to determine if is `NaN`
    assert!(!nan.is_sign_positive() && !nan.is_sign_negative());
}
