fn main() {
    use std::marker::PhantomData;
    
    struct Slice<'a, T:'a> {
        start: *const T,
        end: *const T,
        phantom: PhantomData<&'a T>
    }
}
