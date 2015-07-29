fn main() {
    use std::f32;        let x = 2.0*f32::consts::PI;        let abs_difference = (x.cos() - 1.0).abs();        assert!(abs_difference <= f32::EPSILON);}
