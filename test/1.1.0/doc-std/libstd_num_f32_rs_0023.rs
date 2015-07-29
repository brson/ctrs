fn main() {
    use std::f32;
    
    let ten = 10.0f32;
    let two = 2.0f32;
    
    // log10(10) - 1 == 0
    let abs_difference_10 = (ten.log(10.0) - 1.0).abs();
    
    // log2(2) - 1 == 0
    let abs_difference_2 = (two.log(2.0) - 1.0).abs();
    
    assert!(abs_difference_10 <= f32::EPSILON);
    assert!(abs_difference_2 <= f32::EPSILON);
}
