fn main() {
    use std::env;
    use std::path::PathBuf;
    
    if let Some(path) = env::var_os("PATH") {
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    paths.push(PathBuf::from("/home/xyz/bin"));
    let new_path = env::join_paths(paths).unwrap();
    env::set_var("PATH", &new_path);
    }
}
