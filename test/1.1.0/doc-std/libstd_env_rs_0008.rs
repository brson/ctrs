fn main() {
    use std::env;        let key = "PATH";    match env::var_os(key) {        Some(paths) => {            for path in env::split_paths(&paths) {                println!("'{}'", path.display());            }        }        None => println!("{} is not defined in the environment.", key)    }}
