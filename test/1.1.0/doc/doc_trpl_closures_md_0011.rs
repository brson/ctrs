fn main() {
    let mut num = 5;
    
    {
        let mut add_num = |x: i32| num += x;
    
        add_num(5);
    }
    
    assert_eq!(10, num);
}
