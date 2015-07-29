fn main() {
    let option = Some(5);

    fn foo(x: i32) { }

    match option {

        Some(x) => { foo(x) },

        None => {},

    }

}
