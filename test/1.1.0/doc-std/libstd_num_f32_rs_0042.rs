fn main() {
    use std::f32;
    
    let pi = f32::consts::PI;
    // All angles from horizontal right (+x)
    // 45 deg counter-clockwise
    let x1 = 3.0f32;
    let y1 = -3.0f32;
    
    // 135 deg clockwise
    let x2 = -3.0f32;
    let y2 = 3.0f32;
    
    let abs_difference_1 = (y1.atan2(x1) - (-pi/4.0)).abs();
    let abs_difference_2 = (y2.atan2(x2) - 3.0*pi/4.0).abs();
    
    assert!(abs_difference_1 <= f32::EPSILON);
    assert!(abs_difference_2 <= f32::EPSILON);
}
