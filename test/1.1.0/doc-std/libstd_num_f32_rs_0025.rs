fn main() {
    use std::f32;
    
    let ten = 10.0f32;
    
    // log10(10) - 1 == 0
    let abs_difference = (ten.log10() - 1.0).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
