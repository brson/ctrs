fn main() {
    let mut num = 5;
    {
    let plus_num = |x: i32| x + num;
    
    } // plus_num goes out of scope, borrow of num ends
    
    let y = &mut num;
}
