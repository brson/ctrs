fn main() {
    let v: Vec<&str> = "Mary had a little lamb".rsplitn(2, ' ').collect();
    assert_eq!(v, vec!["lamb", "little", "Mary had a"]);
    
    let v: Vec<&str> = "abc1def2ghi".rsplitn(1, |&: c: char| c.is_numeric()).collect();
    assert_eq!(v, vec!["ghi", "abc1def"]);
    
    let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(2, 'X').collect();
    assert_eq!(v, vec!["leopard", "tiger", "lionX"]);
}
