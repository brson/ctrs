fn main() {
    use std::num::FpCategory;
    use std::f32;
    
    let num = 12.4_f32;
    let inf = f32::INFINITY;
    
    assert_eq!(num.classify(), FpCategory::Normal);
    assert_eq!(inf.classify(), FpCategory::Infinite);
}
