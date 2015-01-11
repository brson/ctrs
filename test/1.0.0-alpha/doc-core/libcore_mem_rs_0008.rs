fn main() {
    use std::mem;
    
    let x = &mut 5i;
    let y = &mut 42i;
    
    mem::swap(x, y);
    
    assert_eq!(42i, *x);
    assert_eq!(5i, *y);
}
