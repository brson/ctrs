fn main() {
    let v: Vec<char> = "abc åäö".chars().collect();
    
    assert_eq!(v, ['a', 'b', 'c', ' ', 'å', 'ä', 'ö']);
}
