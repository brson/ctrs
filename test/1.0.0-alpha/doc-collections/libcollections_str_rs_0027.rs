fn main() {
    let s = "Löwe 老虎 Léopard";
    
    assert_eq!(s.rfind('L'), Some(13));
    assert_eq!(s.rfind('é'), Some(14));
    
    // the second space
    assert_eq!(s.rfind(|&: c: char| c.is_whitespace()), Some(12));
    
    // searches for an occurrence of either `1` or `2`, but neither are found
    let x: &[_] = &['1', '2'];
    assert_eq!(s.rfind(x), None);
}
