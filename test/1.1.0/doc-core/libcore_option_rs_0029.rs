fn main() {
    use std::u16;        let v = vec!(1, 2);    let res: Option<Vec<u16>> = v.iter().map(|&x: &u16|        if x == u16::MAX { None }        else { Some(x + 1) }    ).collect();    assert!(res == Some(vec!(2, 3)));}
