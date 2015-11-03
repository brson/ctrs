fn main() {
    fn plus_one(i: i32) -> i32 {
        i + 1
    }
    
    // without type inference
    let f: fn(i32) -> i32 = plus_one;
    
    // with type inference
    let f = plus_one;
}
