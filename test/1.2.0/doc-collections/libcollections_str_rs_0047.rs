fn main() {
    let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "defXghi"]);
}
