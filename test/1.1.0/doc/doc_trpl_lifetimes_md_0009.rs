struct Foo<'a> {
x: &'a i32,
}

fn main() {
let y = &5;           // -+ y goes into scope
let f = Foo { x: y }; // -+ f goes into scope
// stuff              //  |
//  |
}                         // -+ f and y go out of scope
