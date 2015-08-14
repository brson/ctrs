fn main() {
    let v: Vec<char> = "ASCII żółć 🇨🇭 한".chars().collect();
    
    assert_eq!(v, ['A', 'S', 'C', 'I', 'I', ' ',
        'z', '\u{307}', 'o', '\u{301}', 'ł', 'c', '\u{301}', ' ',
        '\u{1f1e8}', '\u{1f1ed}', ' ', '한']);
}
