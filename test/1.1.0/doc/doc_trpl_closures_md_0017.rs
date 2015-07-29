fn main() {
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }
    
    let answer = call_with_one(&|x| x + 2);
    
    assert_eq!(3, answer);
}
