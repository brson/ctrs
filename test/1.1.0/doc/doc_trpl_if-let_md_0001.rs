fn main() {
    let option = Some(5);

    fn foo(x: i32) { }

    if option.is_some() {

        let x = option.unwrap();

        foo(x);

    }

}
