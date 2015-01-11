struct Foo<'a> {

    x: &'a int,

}



fn main() {

    let y = &5i; // this is the same as `let _y = 5; let y = &_y;

    let f = Foo { x: y };



    println!("{}", f.x);

}

