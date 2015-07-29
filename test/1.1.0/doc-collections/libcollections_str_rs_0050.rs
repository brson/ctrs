fn main() {
    let s = "Löwe 老虎 Léopard";        assert_eq!(s.rfind(|c: char| c.is_whitespace()), Some(12));    assert_eq!(s.rfind(char::is_lowercase), Some(20));}
