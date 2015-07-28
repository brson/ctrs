fn main() {
    let s = "Löwe 老虎 Léopard";
    
    assert_eq!(s.find('L'), Some(0));
    assert_eq!(s.find('é'), Some(14));
    assert_eq!(s.find("Léopard"), Some(13));
    
}
