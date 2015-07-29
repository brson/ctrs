fn main() {
    struct MyTransformResult<T>(Enumerate<Skip<vec::MoveItems<T>>>);
    impl<T> Iterator<(uint, T)> for MyTransformResult<T> { ... }
    
    fn my_transform<T, Iter: Iterator<T>>(iter: Iter) -> MyTransformResult<T> {
        ...
    }
}
