fn main() {
    use std::f64;
    
    let e = f64::consts::E;
    let x = 1.0_f64;
    let f = x.cosh();
    // Solving cosh() at 1 gives this result
    let g = (e*e + 1.0)/(2.0*e);
    let abs_difference = (f - g).abs();
    
    // Same result
    assert!(abs_difference < 1.0e-10);
}
