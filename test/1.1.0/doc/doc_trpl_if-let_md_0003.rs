fn main() {
    let option = Some(5);

    fn foo(x: i32) { }

    fn bar() { }

    if let Some(x) = option {

        foo(x);

    } else {

        bar();

    }

}
