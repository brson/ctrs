mod farm {
    pub fn chicken() { println!("cluck cluck"); }
    pub fn cow() { println!("mooo"); }
    // ...
}

fn main() {
    println!("Hello chicken!");
    ::farm::chicken(); // This compiles now
}
