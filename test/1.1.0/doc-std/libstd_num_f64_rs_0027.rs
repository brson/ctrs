fn main() {
    use std::f64::consts;        let angle = 180.0_f64;        let abs_difference = (angle.to_radians() - consts::PI).abs();        assert!(abs_difference < 1e-10);}
