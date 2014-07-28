fn main() {
    use std::str;
    
    // "abcd"
    let mut v = ['a' as u16, 'b' as u16, 'c' as u16, 'd' as u16];
    // no NULs so no change
    assert_eq!(str::truncate_utf16_at_nul(v), v.as_slice());
    
    // "ab\0d"
    v[2] = 0;
    assert_eq!(str::truncate_utf16_at_nul(v),
               &['a' as u16, 'b' as u16]);
}
