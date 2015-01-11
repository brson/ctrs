fn main() {
    let mut x = 5i;

    

    match x {

        ref mut mr => println!("Got a mutable reference to {}", mr),

    }

}
