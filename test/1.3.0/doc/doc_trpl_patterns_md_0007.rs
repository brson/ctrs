fn main() {
    let some_value: Result<i32, &'static str> = Err("There was an error");
    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occurred"),
    }
}
