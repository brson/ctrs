fn main() {
    let s = "Löwe 老虎 Léopard";
    
    assert_eq!(s.find('L'), Some(0));
    assert_eq!(s.find('é'), Some(14));
    
    // the first space
    assert_eq!(s.find(|c: char| c.is_whitespace()), Some(5));
    
    // neither are found
    assert_eq!(s.find(&['1', '2']), None);
}