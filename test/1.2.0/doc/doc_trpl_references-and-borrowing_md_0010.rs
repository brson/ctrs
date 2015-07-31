fn main() {
    let mut x = 5;
    
    {
    let y = &mut x; // -+ &mut borrow starts here
    *y += 1;        //  |
    }                   // -+ ... and ends here
    
    println!("{}", x);  // <- try to borrow x here
}
