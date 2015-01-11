fn main() {
    fn foo(slice: &mut [int]) {}
    
    let mut vec = vec![1i, 2];
    foo(vec.as_mut_slice());
}
