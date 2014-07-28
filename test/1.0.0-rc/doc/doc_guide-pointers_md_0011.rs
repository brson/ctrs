fn main() {
    let x = 5i;
    let y = &x;
    
    println!("{}", *y);
    println!("{:p}", y);
    println!("{}", y);
}