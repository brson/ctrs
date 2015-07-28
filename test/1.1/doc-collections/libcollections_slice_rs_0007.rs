fn main() {
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.split(|num| *num % 3 == 0) {
        println!("{:?}", group);
    }
}
