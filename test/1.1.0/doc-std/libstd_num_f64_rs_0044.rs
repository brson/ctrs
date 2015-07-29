fn main() {
    let x = 7.0_f64;
    
    // e^(ln(7)) - 1
    let abs_difference = (x.ln().exp_m1() - 6.0).abs();
    
    assert!(abs_difference < 1e-10);
}
