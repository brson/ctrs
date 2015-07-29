fn main() {
    let v = [10, 40, 30];    assert_eq!(Some(&40), v.get(1));    assert_eq!(None, v.get(3));}
