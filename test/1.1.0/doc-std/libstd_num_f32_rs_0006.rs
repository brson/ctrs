fn main() {
    let f = 3.99_f32;
    let g = 3.0_f32;
    
    assert_eq!(f.floor(), 3.0);
    assert_eq!(g.floor(), 3.0);
}
