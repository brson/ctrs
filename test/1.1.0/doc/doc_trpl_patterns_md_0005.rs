fn main() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }
    
    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }
}
