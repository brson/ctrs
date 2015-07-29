fn main() {
    use std::env;        let key = "KEY";    env::set_var(key, "VALUE");    assert_eq!(env::var(key), Ok("VALUE".to_string()));        env::remove_var(key);    assert!(env::var(key).is_err());}
