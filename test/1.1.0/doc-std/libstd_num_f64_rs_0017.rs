fn main() {
    let x = 2.0_f64;    let abs_difference = (x.powi(2) - x*x).abs();        assert!(abs_difference < 1e-10);}
