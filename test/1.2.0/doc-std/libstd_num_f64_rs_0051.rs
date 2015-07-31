fn main() {
    use std::f64;
    
    let e = f64::consts::E;
    let f = e.tanh().atanh();
    
    let abs_difference = (f - e).abs();
    
    assert!(abs_difference < 1.0e-10);
}
