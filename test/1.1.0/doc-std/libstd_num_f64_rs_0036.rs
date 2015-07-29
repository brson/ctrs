fn main() {
    use std::f64;        let x = f64::consts::PI/2.0;        let abs_difference = (x.sin() - 1.0).abs();        assert!(abs_difference < 1e-10);}
