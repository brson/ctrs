fn main() {
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_works() {
    assert_eq!("Hello", "world");
    }
}
