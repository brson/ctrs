fn main() {
    let home: &'static str = env!("HOME");
    println!("the home directory at the time of compiling was: {}", home);
}
