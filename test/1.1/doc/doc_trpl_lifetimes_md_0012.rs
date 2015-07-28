fn main() {
    static FOO: i32 = 5;

    let x: &'static i32 = &FOO;

}
