fn main() {
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.slice(0, 1), "L");
    
    assert_eq!(s.slice(1, 9), "öwe 老");
    
    // these will fail:
    // byte 2 lies within `ö`:
    // s.slice(2, 3);
    
    // byte 8 lies within `老`
    // s.slice(1, 8);
    
    // byte 100 is outside the string
    // s.slice(3, 100);
}
