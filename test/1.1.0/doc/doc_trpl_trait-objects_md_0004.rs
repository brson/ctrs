trait Foo { fn method(&self) -> String; }

impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }

impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }



fn do_something(x: &Foo) {

    x.method();

}



fn main() {

    let x = 5u8;

    do_something(&x as &Foo);

}

