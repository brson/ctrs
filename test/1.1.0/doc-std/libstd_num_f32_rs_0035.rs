fn main() {
    use std::f32;
    
    let x = 2.0f32;
    let y = 3.0f32;
    
    // sqrt(x^2 + y^2)
    let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
