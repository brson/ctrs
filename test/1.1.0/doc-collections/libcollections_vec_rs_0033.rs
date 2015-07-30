fn main() {
    let v = vec!["a".to_string(), "b".to_string()];
    for s in v.into_iter() {
    // s has type String, not &String
    println!("{}", s);
    }
}
