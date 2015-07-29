fn main() {
    let x = 7.0f64;
    
    // e^(ln(7)) - 1
    let abs_difference = x.ln().exp_m1().abs_sub(6.0);
    
    assert!(abs_difference < 1e-10);
}
