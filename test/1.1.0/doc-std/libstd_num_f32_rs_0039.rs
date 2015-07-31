fn main() {
    use std::f32;
    
    let f = f32::consts::PI / 2.0;
    
    // asin(sin(pi/2))
    let abs_difference = f.sin().asin().abs_sub(f32::consts::PI / 2.0);
    
    assert!(abs_difference <= f32::EPSILON);
}
