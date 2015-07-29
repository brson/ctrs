fn main() {
    let positive = 4.0_f64;    let negative = -4.0_f64;        let abs_difference = (positive.sqrt() - 2.0).abs();        assert!(abs_difference < 1e-10);    assert!(negative.sqrt().is_nan());}
