fn main() {
    let v: Vec<&str> = "A.B.".split_terminator('.').collect();    assert_eq!(v, ["A", "B"]);        let v: Vec<&str> = "A..B..".split_terminator(".").collect();    assert_eq!(v, ["A", "", "B", ""]);}
