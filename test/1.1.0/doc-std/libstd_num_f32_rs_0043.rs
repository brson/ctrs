fn main() {
    use std::f32;
    
    let x = f32::consts::PI/4.0;
    let f = x.sin_cos();
    
    let abs_difference_0 = (f.0 - x.sin()).abs();
    let abs_difference_1 = (f.1 - x.cos()).abs();
    
    assert!(abs_difference_0 <= f32::EPSILON);
    assert!(abs_difference_0 <= f32::EPSILON);
}
