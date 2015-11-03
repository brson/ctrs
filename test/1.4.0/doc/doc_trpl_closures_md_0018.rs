fn main() {
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }
    
    fn add_one(i: i32) -> i32 {
        i + 1
    }
    
    let f = add_one;
    
    let answer = call_with_one(&f);
    
    assert_eq!(2, answer);
}
