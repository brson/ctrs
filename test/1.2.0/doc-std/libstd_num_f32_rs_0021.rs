fn main() {
    use std::f32;
    
    let f = 2.0f32;
    
    // 2^2 - 4 == 0
    let abs_difference = (f.exp2() - 4.0).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
