fn main() {
    let n = 0b0101000u16;
    
    assert_eq!(n.trailing_zeros(), 3);
}
