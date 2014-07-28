fn main() {
    use std::task::spawn;
    
    // Print something profound in a different task using a named function
    fn print_message() { println!("I am running in a different task!"); }
    spawn(print_message);
    
    // Print something profound in a different task using a `proc` expression
    // The `proc` expression evaluates to an (unnamed) owned closure.
    // That closure will call `println!(...)` when the spawned task runs.
    
    spawn(proc() println!("I am also running in a different task!") );
}