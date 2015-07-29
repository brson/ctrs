fn main() {
    let ten = 10.0_f64;
    let two = 2.0_f64;
    
    // log10(10) - 1 == 0
    let abs_difference_10 = (ten.log(10.0) - 1.0).abs();
    
    // log2(2) - 1 == 0
    let abs_difference_2 = (two.log(2.0) - 1.0).abs();
    
    assert!(abs_difference_10 < 1e-10);
    assert!(abs_difference_2 < 1e-10);
}
