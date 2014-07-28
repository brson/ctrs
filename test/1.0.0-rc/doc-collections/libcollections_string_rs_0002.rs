fn main() {
    // 𝄞music
    let mut v = [0xD834, 0xDD1E, 0x006d, 0x0075,
                 0x0073, 0x0069, 0x0063];
    assert_eq!(String::from_utf16(v), Some("𝄞music".to_string()));
    
    // 𝄞mu<invalid>ic
    v[4] = 0xD800;
    assert_eq!(String::from_utf16(v), None);
}
