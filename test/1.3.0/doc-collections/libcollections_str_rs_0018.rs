fn main() {
    let some_words = " Mary   had\ta\u{2009}little  \n\t lamb";
    let v: Vec<&str> = some_words.split_whitespace().collect();
    
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);
}
