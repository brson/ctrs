fn main() {
    let v: Vec<&str> = "abc1def2ghi".splitn(2, |c: char| c.is_numeric()).collect();    assert_eq!(v, ["abc", "def2ghi"]);}
