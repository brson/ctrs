fn takes_slice(slice: &str) {

    println!("Got: {}", slice);

}



fn main() {

    let s = "Hello".to_string();

    takes_slice(&s);

}

