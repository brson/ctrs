fn main() {
    use std::f64;
    
    let pi = f64::consts::PI;
    // All angles from horizontal right (+x)
    // 45 deg counter-clockwise
    let x1 = 3.0_f64;
    let y1 = -3.0_f64;
    
    // 135 deg clockwise
    let x2 = -3.0_f64;
    let y2 = 3.0_f64;
    
    let abs_difference_1 = (y1.atan2(x1) - (-pi/4.0)).abs();
    let abs_difference_2 = (y2.atan2(x2) - 3.0*pi/4.0).abs();
    
    assert!(abs_difference_1 < 1e-10);
    assert!(abs_difference_2 < 1e-10);
}
