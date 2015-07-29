fn main() {
    fn foo(c: &[int]) { ... }
    fn foo(c: &Vec<int>) { ... }
    fn foo(c: &SomeOtherCollection<int>) { ... }
}
