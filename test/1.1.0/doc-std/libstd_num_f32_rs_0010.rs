fn main() {
    use std::f32;
    
    let x = 3.5_f32;
    let y = -3.5_f32;
    let abs_difference_x = (x.fract() - 0.5).abs();
    let abs_difference_y = (y.fract() - (-0.5)).abs();
    
    assert!(abs_difference_x <= f32::EPSILON);
    assert!(abs_difference_y <= f32::EPSILON);
}
