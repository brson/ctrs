fn main() {
    let good_result: Result<int, int> = Ok(10);
    let bad_result: Result<int, int> = Err(10);
    
    // The `is_ok` and `is_err` methods do what they say.
    assert!(good_result.is_ok() && !good_result.is_err());
    assert!(bad_result.is_err() && !bad_result.is_ok());
    
    // `map` consumes the `Result` and produces another.
    let good_result: Result<int, int> = good_result.map(|i| i + 1);
    let bad_result: Result<int, int> = bad_result.map(|i| i - 1);
    
    // Use `and_then` to continue the computation.
    let good_result: Result<bool, int> = good_result.and_then(|i| Ok(i == 11));
    
    // Use `or_else` to handle the error.
    let bad_result: Result<int, int> = bad_result.or_else(|i| Ok(11));
    
    // Consume the result and return the contents with `unwrap`.
    let final_awesome_result = good_result.ok().unwrap();
}
