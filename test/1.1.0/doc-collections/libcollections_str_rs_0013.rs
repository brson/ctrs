fn main() {
    let v: Vec<&str> = "abc1def2ghi3".split_terminator(|c: char| c.is_numeric()).collect();    assert_eq!(v, ["abc", "def", "ghi"]);}
