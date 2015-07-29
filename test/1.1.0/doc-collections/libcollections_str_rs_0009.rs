fn main() {
    let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();    assert_eq!(v, ["abc", "def", "ghi"]);        let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();    assert_eq!(v, ["lion", "tiger", "leopard"]);}
