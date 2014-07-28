fn main() {
    // the struct owns the objects contained in the `x` and `y` fields
    struct Foo { x: int, y: Box<int> }
    
    {
        // `a` is the owner of the struct, and thus the owner of the struct's fields
        let a = Foo { x: 5, y: box 10 };
    }
    // when `a` goes out of scope, the destructor for the `Box<int>` in the struct's
    // field is called
    
    // `b` is mutable, and the mutability is inherited by the objects it owns
    let mut b = Foo { x: 5, y: box 10 };
    b.x = 10;
}