fn main() {
    use std::borrow::Cow;
    
    let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    
    let hello = cow.to_mut();
    
    assert_eq!(hello, &[1, 2, 3]);
}
