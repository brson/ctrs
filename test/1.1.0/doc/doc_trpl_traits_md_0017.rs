fn main() {
    use std::fmt::Debug;
    
    fn bar<T, K>(x: T, y: K)
        where T: Clone,
              K: Clone + Debug {
    
        x.clone();
        y.clone();
        println!("{:?}", y);
    }
}
