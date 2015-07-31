fn main() {
    use std::f32;
    
    let x = f32::consts::PI/2.0;
    
    let abs_difference = (x.sin() - 1.0).abs();
    
    assert!(abs_difference <= f32::EPSILON);
}
