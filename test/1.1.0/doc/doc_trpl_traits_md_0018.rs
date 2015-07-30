fn main() {
    trait ConvertTo<Output> {
    fn convert(&self) -> Output;
    }
    
    impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
    }
    
    // can be called with T == i32
    fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
    }
    
    // can be called with T == i64
    fn inverse<T>() -> T
    // this is using ConvertTo as if it were "ConvertFrom<i32>"
    where i32: ConvertTo<T> {
    1i32.convert()
    }
}
