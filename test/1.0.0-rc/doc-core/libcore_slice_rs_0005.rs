fn main() {
    use std::slice::MutableCloneableVector;
    
    let mut dst = [0i, 0, 0];
    let src = [1i, 2];
    
    assert!(dst.copy_from(src) == 2);
    assert!(dst == [1, 2, 0]);
    
    let src2 = [3i, 4, 5, 6];
    assert!(dst.copy_from(src2) == 3);
    assert!(dst == [3i, 4, 5]);
}
