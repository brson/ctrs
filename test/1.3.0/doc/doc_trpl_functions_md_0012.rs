fn main() {
    let mut y = 5;
    
    let x = (y = 6);  // x has the value `()`, not `6`
}
