trait Foo { fn method(&self) -> String; }

impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }

impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }



fn do_something(x: &Foo) {

    x.method();

}



fn main() {

    let x = "Hello".to_string();

    do_something(&x);

}

