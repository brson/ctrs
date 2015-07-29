fn main() {
    use std::f64;        let x = 3.5_f64;    let y = -3.5_f64;        let abs_difference_x = (x.abs() - x).abs();    let abs_difference_y = (y.abs() - (-y)).abs();        assert!(abs_difference_x < 1e-10);    assert!(abs_difference_y < 1e-10);        assert!(f64::NAN.abs().is_nan());}
