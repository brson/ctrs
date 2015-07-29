fn main() {
    let v = [10, 40, 30, 20, 60, 50];    for group in v.splitn(2, |num| *num % 3 == 0) {        println!("{:?}", group);    }}
