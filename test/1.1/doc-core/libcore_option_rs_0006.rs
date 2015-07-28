fn main() {
    let num_as_str: Option<String> = Some("10".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `num_as_str` on the stack.
    let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
    println!("still can print num_as_str: {:?}", num_as_str);
}
