struct BigStruct {
one: i32,
two: i32,
// etc
one_hundred: i32,
}

fn foo(x: Box<BigStruct>) -> Box<BigStruct> {
Box::new(*x)
}

fn main() {
let x = Box::new(BigStruct {
one: 1,
two: 2,
one_hundred: 100,
});

let y = foo(x);
}
