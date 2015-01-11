fn main() {
    let x = &5i;

    

    match x {

        &val => println!("Got a value: {}", val),

    }

}
