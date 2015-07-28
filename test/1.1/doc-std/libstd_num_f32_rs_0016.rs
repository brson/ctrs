fn main() {
    use std::f32;
    
    let x = 2.0_f32;
    let abs_difference = (x.recip() - (1.0/x)).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
