fn main() {
    let num = 5;
    let plus_num = |x: i32| x + num;
    
    assert_eq!(10, plus_num(5));
}
