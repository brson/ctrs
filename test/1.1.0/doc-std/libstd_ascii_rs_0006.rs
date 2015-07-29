fn main() {
    use std::ascii;        let escaped = ascii::escape_default(b'0').next().unwrap();    assert_eq!(b'0', escaped);        let mut escaped = ascii::escape_default(b'\t');        assert_eq!(b'\\', escaped.next().unwrap());    assert_eq!(b't', escaped.next().unwrap());}
