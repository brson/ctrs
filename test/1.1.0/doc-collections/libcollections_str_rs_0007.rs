fn main() {
    let v: Vec<(usize, char)> = "abc".char_indices().collect();
    let b = vec![(0, 'a'), (1, 'b'), (2, 'c')];
    
    assert_eq!(v, b);
}
