fn main() {
    let vec = vec![1u, 2, 3, 4];
    for x in vec.iter().rev() {
       println!("vec contained {}", x);
    }
}
