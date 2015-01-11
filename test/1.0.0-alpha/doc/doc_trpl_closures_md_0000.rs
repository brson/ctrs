fn main() {
    let add_one = |&: x| { 1 + x };

    

    println!("The sum of 5 plus 1 is {}.", add_one(5));

}
