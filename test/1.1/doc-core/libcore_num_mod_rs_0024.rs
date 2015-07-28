fn main() {
    let n = 0x0123456789ABCDEFu64;
    
    if cfg!(target_endian = "little") {
        assert_eq!(u64::from_le(n), n)
    } else {
        assert_eq!(u64::from_le(n), n.swap_bytes())
    }
}
