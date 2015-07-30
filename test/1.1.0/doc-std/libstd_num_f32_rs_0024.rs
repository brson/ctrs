fn main() {
    use std::f32;
    
    let two = 2.0f32;
    
    // log2(2) - 1 == 0
    let abs_difference = (two.log2() - 1.0).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
