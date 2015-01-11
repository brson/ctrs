fn main() {
    use std::num::Int;
    
    let n = 0x0123456789ABCDEFu64;
    
    if cfg!(target_endian = "big") {
        assert_eq!(Int::from_be(n), n)
    } else {
        assert_eq!(Int::from_be(n), n.swap_bytes())
    }
}
