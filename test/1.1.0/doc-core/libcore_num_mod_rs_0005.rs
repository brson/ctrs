fn main() {
    let n = 0x0123456789ABCDEFu64;    let m = 0xDEF0123456789ABCu64;        assert_eq!(n.rotate_right(12), m);}
