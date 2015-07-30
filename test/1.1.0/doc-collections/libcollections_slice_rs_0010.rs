fn main() {
    let v = &[1, 2, 3, 4];
    for win in v.windows(2) {
    println!("{:?}", win);
    }
}
