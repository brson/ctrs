fn main() {
    let numbers = [0i, 1i, 2i];
    for &x in numbers.iter() {
        println!("{} is a number!", x);
    }
}
