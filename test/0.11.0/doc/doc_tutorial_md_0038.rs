fn main() {
    let mytup: (int, int, f64) = (10, 20, 30.0);
    match mytup {
      (a, b, c) => println!("{}", a + b + (c as int))
    }
}