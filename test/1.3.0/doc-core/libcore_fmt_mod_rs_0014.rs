fn main() {
    let x = &42;
    
    let address = format!("{:p}", x); // this produces something like '0x7f06092ac6d0'
}
