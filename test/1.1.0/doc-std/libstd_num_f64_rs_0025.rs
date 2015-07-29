fn main() {
    let ten = 10.0_f64;
    
    // log10(10) - 1 == 0
    let abs_difference = (ten.log10() - 1.0).abs();
    
    assert!(abs_difference < 1e-10);
}
