fn main() {
    struct MyTup(int, int, f64);
    let mytup: MyTup = MyTup(10, 20, 30.0);
    match mytup {
      MyTup(a, b, c) => println!("{}", a + b + (c as int))
    }
}