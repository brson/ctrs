fn main() {
    let x = Some(4);    assert_eq!(x.iter().next(), Some(&4));        let x: Option<u32> = None;    assert_eq!(x.iter().next(), None);}
