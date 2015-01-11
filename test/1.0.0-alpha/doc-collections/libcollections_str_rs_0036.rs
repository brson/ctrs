fn main() {
    let some_words = " Mary   had\ta little  \n\t lamb";
    let v: Vec<&str> = some_words.words().collect();
    assert_eq!(v, vec!["Mary", "had", "a", "little", "lamb"]);
}
