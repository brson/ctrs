fn main() {
    let v: Vec<&str> = "abc1def2ghi3".rsplit_terminator(|c: char| c.is_numeric()).collect();    assert_eq!(v, ["ghi", "def", "abc"]);}
