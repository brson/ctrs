fn main() {
    let num_as_str: Option<String> = Some("10".to_string());
    // `Option::map` takes self *by value*, consuming `num_as_str`
    let num_as_int: Option<usize> = num_as_str.map(|n| n.len());
}
