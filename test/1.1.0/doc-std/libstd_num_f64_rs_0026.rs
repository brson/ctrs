fn main() {
    use std::f64::consts;        let angle = consts::PI;        let abs_difference = (angle.to_degrees() - 180.0).abs();        assert!(abs_difference < 1e-10);}
