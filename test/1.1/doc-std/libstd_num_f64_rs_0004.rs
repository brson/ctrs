fn main() {
    use std::num::FpCategory;
    use std::f64;
    
    let num = 12.4_f64;
    let inf = f64::INFINITY;
    
    assert_eq!(num.classify(), FpCategory::Normal);
    assert_eq!(inf.classify(), FpCategory::Infinite);
}
