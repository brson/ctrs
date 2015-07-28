fn main() {
    use std::f32;
    
    let min = f32::MIN_POSITIVE; // 1.17549435e-38f64
    let max = f32::MAX;
    let lower_than_min = 1.0e-40_f32;
    let zero = 0.0f32;
    
    assert!(min.is_normal());
    assert!(max.is_normal());
    
    assert!(!zero.is_normal());
    assert!(!f32::NAN.is_normal());
    assert!(!f32::INFINITY.is_normal());
    // Values between `0` and `min` are Subnormal.
    assert!(!lower_than_min.is_normal());
}
