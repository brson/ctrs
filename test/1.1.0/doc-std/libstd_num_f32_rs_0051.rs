fn main() {
    use std::f32;        let e = f32::consts::E;    let f = e.tanh().atanh();        let abs_difference = f.abs_sub(e);        assert!(abs_difference <= f32::EPSILON);}
