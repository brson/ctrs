fn main() {
    let f = 3.3_f64;
    let g = -3.3_f64;
    
    assert_eq!(f.round(), 3.0);
    assert_eq!(g.round(), -3.0);
}
