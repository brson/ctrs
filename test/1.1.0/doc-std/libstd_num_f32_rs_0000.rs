fn main() {
    use std::f32;        let nan = f32::NAN;    let f = 7.0_f32;        assert!(nan.is_nan());    assert!(!f.is_nan());}
