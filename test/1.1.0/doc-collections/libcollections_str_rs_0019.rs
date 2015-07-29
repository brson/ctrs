fn main() {
    let v: Vec<&str> = "abc1def2ghi".rsplitn(2, |c: char| c.is_numeric()).collect();
    assert_eq!(v, ["ghi", "abc1def"]);
}
