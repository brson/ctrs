fn main() {
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.slice_chars(0, 4), "Löwe");
    assert_eq!(s.slice_chars(5, 7), "老虎");
}
