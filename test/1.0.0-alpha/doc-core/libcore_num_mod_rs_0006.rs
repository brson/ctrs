fn main() {
    use std::num::Int;
    
    let n = 0x0123456789ABCDEFu64;
    let m = 0xEFCDAB8967452301u64;
    
    assert_eq!(n.swap_bytes(), m);
}
