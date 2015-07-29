fn main() {
    use std::f64;
    
    let x = f64::consts::E - 1.0;
    
    // ln(1 + (e - 1)) == ln(e) == 1
    let abs_difference = (x.ln_1p() - 1.0).abs();
    
    assert!(abs_difference < 1e-10);
}
