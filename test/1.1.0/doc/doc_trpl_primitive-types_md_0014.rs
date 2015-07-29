fn main() {
    fn foo(x: i32) -> i32 { x }
    
    let x: fn(i32) -> i32 = foo;
}
