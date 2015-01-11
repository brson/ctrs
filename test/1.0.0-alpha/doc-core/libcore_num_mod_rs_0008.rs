fn main() {
    use std::num::Int;
    
    let n = 0x0123456789ABCDEFu64;
    
    if cfg!(target_endian = "little") {
        assert_eq!(Int::from_le(n), n)
    } else {
        assert_eq!(Int::from_le(n), n.swap_bytes())
    }
}
