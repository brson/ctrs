fn main() {
    let mut v = vec!["foo".to_string(), "bar".to_string(),
                     "baz".to_string(), "qux".to_string()];
    
    assert_eq!(v.swap_remove(1), Some("bar".to_string()));
    assert_eq!(v, vec!["foo".to_string(), "qux".to_string(), "baz".to_string()]);
    
    assert_eq!(v.swap_remove(0), Some("foo".to_string()));
    assert_eq!(v, vec!["baz".to_string(), "qux".to_string()]);
    
    assert_eq!(v.swap_remove(2), None);
}
