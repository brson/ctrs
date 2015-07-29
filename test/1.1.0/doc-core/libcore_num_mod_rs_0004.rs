fn main() {
    let n = 0x0123456789ABCDEFu64;    let m = 0x3456789ABCDEF012u64;        assert_eq!(n.rotate_left(12), m);}
