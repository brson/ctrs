fn main() {
    let path: &'static str = env!("PATH");
    println!("the $PATH variable at the time of compiling was: {}", path);
}
