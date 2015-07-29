fn main() {
    // Excerpt from std::str
    
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_eq() {
            assert!((eq(&"".to_owned(), &"".to_owned())));
            assert!((eq(&"foo".to_owned(), &"foo".to_owned())));
            assert!((!eq(&"foo".to_owned(), &"bar".to_owned())));
        }
    }
}
