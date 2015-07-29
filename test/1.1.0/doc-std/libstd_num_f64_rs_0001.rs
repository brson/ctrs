fn main() {
    use std::f64;        let f = 7.0f64;    let inf = f64::INFINITY;    let neg_inf = f64::NEG_INFINITY;    let nan = f64::NAN;        assert!(!f.is_infinite());    assert!(!nan.is_infinite());        assert!(inf.is_infinite());    assert!(neg_inf.is_infinite());}
