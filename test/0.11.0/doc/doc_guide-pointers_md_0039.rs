struct BigStruct {
    one: int,
    two: int,
    // etc
    one_hundred: int,
}

fn foo(x: Box<BigStruct>) -> BigStruct {
    return *x;
}

fn main() {
    let x = box BigStruct {
        one: 1,
        two: 2,
        one_hundred: 100,
    };

    let y = box foo(x);
}
