fn main() {
    let x = "    a  b c".to_string();
    let d: Vec<_> = x.split(' ').collect();
    
    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
}
