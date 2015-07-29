fn main() {
    let v = &[1, 2, 3, 4, 5];    for win in v.chunks(2) {        println!("{:?}", win);    }}
