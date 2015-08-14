fn main() {
    let plus_two = |x| {
        let mut result: i32 = x;
    
        result += 1;
        result += 1;
    
        result
    };
    
    assert_eq!(4, plus_two(2));
}
