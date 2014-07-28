fn main() {
    use std::mem;
    
    struct S<T> { x: *const () }
    fn get<T>(s: &S<T>, v: T) {
       unsafe {
           let x: fn(T) = mem::transmute(s.x);
           x(v)
       }
    }
}
