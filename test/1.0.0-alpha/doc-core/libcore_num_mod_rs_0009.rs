fn main() {
    use std::num::Int;
    
    let n = 0x0123456789ABCDEFu64;
    
    if cfg!(target_endian = "big") {
        assert_eq!(n.to_be(), n)
    } else {
        assert_eq!(n.to_be(), n.swap_bytes())
    }
}
