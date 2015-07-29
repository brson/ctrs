fn main() {
    let x = 3.5_f64;
    let y = -3.5_f64;
    let abs_difference_x = (x.fract() - 0.5).abs();
    let abs_difference_y = (y.fract() - (-0.5)).abs();
    
    assert!(abs_difference_x < 1e-10);
    assert!(abs_difference_y < 1e-10);
}
