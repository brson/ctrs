fn main() {
    let s = "Löwe 老虎 Léopard";
    
    assert_eq!(s.find_str("老虎 L"), Some(6));
    assert_eq!(s.find_str("muffin man"), None);
}
