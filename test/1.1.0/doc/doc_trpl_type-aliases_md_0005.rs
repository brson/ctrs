fn main() {
    use std::result;
    
    enum ConcreteError {
        Foo,
        Bar,
    }
    
    type Result<T> = result::Result<T, ConcreteError>;
}
