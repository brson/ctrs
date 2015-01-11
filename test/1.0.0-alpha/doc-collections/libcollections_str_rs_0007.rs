fn main() {
    let v: Vec<&str> = "Mary had a little lambda".splitn(2, ' ').collect();
    assert_eq!(v, vec!["Mary", "had", "a little lambda"]);
    
    let v: Vec<&str> = "abc1def2ghi".splitn(1, |&: c: char| c.is_numeric()).collect();
    assert_eq!(v, vec!["abc", "def2ghi"]);
    
    let v: Vec<&str> = "lionXXtigerXleopard".splitn(2, 'X').collect();
    assert_eq!(v, vec!["lion", "", "tigerXleopard"]);
    
    let v: Vec<&str> = "abcXdef".splitn(0, 'X').collect();
    assert_eq!(v, vec!["abcXdef"]);
    
    let v: Vec<&str> = "".splitn(1, 'X').collect();
    assert_eq!(v, vec![""]);
}
