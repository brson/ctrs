fn main() {
    // implicit
    fn foo(x: &i32) {
    }
    
    // explicit
    fn bar<'a>(x: &'a i32) {
    }
}
