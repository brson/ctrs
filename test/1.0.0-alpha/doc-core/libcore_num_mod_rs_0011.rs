fn main() {
    use std::num::Int;
    
    assert_eq!(5u16.checked_add(65530), Some(65535));
    assert_eq!(6u16.checked_add(65530), None);
}
