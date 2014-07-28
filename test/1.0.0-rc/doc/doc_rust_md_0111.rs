fn main() {
    enum List<X> { Nil, Cons(X, Box<List<X>>) }
    
    let x: List<int> = Cons(10, box Cons(11, box Nil));
    
    match x {
        Cons(_, box Nil) => fail!("singleton list"),
        Cons(..)         => return,
        Nil              => fail!("empty list")
    }
}