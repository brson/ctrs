fn main() {
    use std::mem;
    
    let mut v: Vec<i32> = Vec::new();
    
    mem::replace(&mut v, Vec::new());
}
