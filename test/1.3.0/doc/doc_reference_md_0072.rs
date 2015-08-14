fn main() {
    let x: i32 = { println!("Hello."); 5 };
    
    assert_eq!(5, x);
}
