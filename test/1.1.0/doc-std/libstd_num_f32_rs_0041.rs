fn main() {
    use std::f32;
    
    let f = 1.0f32;
    
    // atan(tan(1))
    let abs_difference = f.tan().atan().abs_sub(1.0);
    
    assert!(abs_difference <= f32::EPSILON);
}
