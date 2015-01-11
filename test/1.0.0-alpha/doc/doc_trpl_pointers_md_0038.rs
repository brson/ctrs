use std::boxed::Box;

struct BigStruct {

    one: int,

    two: int,

    // etc

    one_hundred: int,

}



fn foo(x: Box<BigStruct>) -> Box<BigStruct> {

    return Box::new(*x);

}



fn main() {

    let x = Box::new(BigStruct {

        one: 1,

        two: 2,

        one_hundred: 100,

    });



    let y = foo(x);

}

