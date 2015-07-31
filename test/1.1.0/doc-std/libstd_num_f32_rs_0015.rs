fn main() {
    use std::f32;
    
    let m = 10.0_f32;
    let x = 4.0_f32;
    let b = 60.0_f32;
    
    // 100.0
    let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
