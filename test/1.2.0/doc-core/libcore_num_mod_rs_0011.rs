fn main() {
    let n = 0x0123456789ABCDEFu64;
    
    if cfg!(target_endian = "little") {
    assert_eq!(n.to_le(), n)
    } else {
    assert_eq!(n.to_le(), n.swap_bytes())
    }
}
