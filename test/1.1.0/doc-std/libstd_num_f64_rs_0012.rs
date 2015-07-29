fn main() {
    use std::f64;        let f = 3.5_f64;        assert_eq!(f.signum(), 1.0);    assert_eq!(f64::NEG_INFINITY.signum(), -1.0);        assert!(f64::NAN.signum().is_nan());}
