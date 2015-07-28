fn main() {
    use std::f32;
    
    let one = 1.0f32;
    // e^1
    let e = one.exp();
    
    // ln(e) - 1 == 0
    let abs_difference = (e.ln() - 1.0).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
