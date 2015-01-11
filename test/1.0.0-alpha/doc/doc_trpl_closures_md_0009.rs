fn main() {
    fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {

      f(x) + f(x)

    }

}
