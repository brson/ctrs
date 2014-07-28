fn main() {
    use std::task::spawn;
    
    // proc is the closure which will be spawned.
    spawn(proc() {
        println!("I'm a new task")
    });
}