fn main() {
    use std::boxed::Box;

    enum List<T> {

        Nil,

        Cons(T, Box<List<T>>)

    }

    

    let a: List<int> = List::Cons(7, Box::new(List::Cons(13, Box::new(List::Nil))));

}
