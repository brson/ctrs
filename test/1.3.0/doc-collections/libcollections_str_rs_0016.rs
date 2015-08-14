fn main() {
    let v: Vec<(usize, char)> = "A🇨🇭".char_indices().collect();
    let b = vec![(0, 'A'), (1, '\u{1f1e8}'), (5, '\u{1f1ed}')];
    
    assert_eq!(v, b);
}
