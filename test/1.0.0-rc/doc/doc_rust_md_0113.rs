fn main() {
    fn process_pair(a: int, b: int) { }
    fn process_ten() { }
    
    enum List<X> { Nil, Cons(X, Box<List<X>>) }
    
    let x: List<int> = Cons(10, box Cons(11, box Nil));
    
    match x {
        Cons(a, box Cons(b, _)) => {
            process_pair(a, b);
        }
        Cons(10, _) => {
            process_ten();
        }
        Nil => {
            return;
        }
        _ => {
            fail!();
        }
    }
}