fn main() {
    let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["ghi", "abc1def"]);
}
