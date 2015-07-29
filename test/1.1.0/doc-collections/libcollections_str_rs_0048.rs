fn main() {
    let s = "Löwe 老虎 Léopard";
    let x: &[_] = &['1', '2'];
    
    assert_eq!(s.find(x), None);
}
