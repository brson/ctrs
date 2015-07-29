fn main() {
    let v: Vec<&str> = "abc1def2ghi".rsplit(|c: char| c.is_numeric()).collect();
    assert_eq!(v, ["ghi", "def", "abc"]);
    
    let v: Vec<&str> = "lionXtigerXleopard".rsplit(char::is_uppercase).collect();
    assert_eq!(v, ["leopard", "tiger", "lion"]);
}
