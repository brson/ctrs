fn main() {
    use std::f32;
    
    let x = 1.0f32;
    let f = x.cosh().acosh();
    
    let abs_difference = (f - x).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
