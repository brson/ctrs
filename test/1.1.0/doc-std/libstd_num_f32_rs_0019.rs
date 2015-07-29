fn main() {
    use std::f32;
    
    let positive = 4.0_f32;
    let negative = -4.0_f32;
    
    let abs_difference = (positive.sqrt() - 2.0).abs();
    
    assert!(abs_difference <= f32::EPSILON);
    assert!(negative.sqrt().is_nan());
}
