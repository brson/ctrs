fn main() {
    let s = "Löwe 老虎 Léopard";        assert_eq!(s.find(|c: char| c.is_whitespace()), Some(5));    assert_eq!(s.find(char::is_lowercase), Some(1));}
