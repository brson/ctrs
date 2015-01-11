fn main() {
    trait Seq<T> {

       fn len(&self) -> uint;

       fn elt_at(&self, n: uint) -> T;

       fn iter<F>(&self, F) where F: Fn(T);

    }

}
