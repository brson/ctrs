fn main() {
    fn plus_one(i: i32) -> i32 { i + 1 }
    let f = plus_one;
    let six = f(5);
}
