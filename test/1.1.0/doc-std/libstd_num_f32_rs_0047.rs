fn main() {
    use std::f32;
    
    let e = f32::consts::E;
    let x = 1.0f32;
    let f = x.cosh();
    // Solving cosh() at 1 gives this result
    let g = (e*e + 1.0)/(2.0*e);
    let abs_difference = f.abs_sub(g);
    
    // Same result
    assert!(abs_difference <= f32::EPSILON);
}
