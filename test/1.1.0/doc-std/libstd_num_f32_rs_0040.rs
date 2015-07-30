fn main() {
    use std::f32;
    
    let f = f32::consts::PI / 4.0;
    
    // acos(cos(pi/4))
    let abs_difference = f.cos().acos().abs_sub(f32::consts::PI / 4.0);
    
    assert!(abs_difference <= f32::EPSILON);
}
