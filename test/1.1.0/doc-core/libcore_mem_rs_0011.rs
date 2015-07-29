fn main() {
    use std::mem;        let x = &mut 5;    let y = &mut 42;        mem::swap(x, y);        assert_eq!(42, *x);    assert_eq!(5, *y);}
