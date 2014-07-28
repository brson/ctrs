fn main() {
    enum List<T> {
      Nil,
      Cons(T, Box<List<T>>)
    }
    
    let a: List<int> = Cons(7, box Cons(13, box Nil));
}