fn main() {
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, vec!["Mary", "had", "a", "little", "lamb"]);
    
    let v: Vec<&str> = "abc1def2ghi".split(|&: c: char| c.is_numeric()).collect();
    assert_eq!(v, vec!["abc", "def", "ghi"]);
    
    let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
    assert_eq!(v, vec!["lion", "", "tiger", "leopard"]);
    
    let v: Vec<&str> = "".split('X').collect();
    assert_eq!(v, vec![""]);
}
