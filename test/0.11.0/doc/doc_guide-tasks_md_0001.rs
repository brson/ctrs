fn main() {
    use std::task::spawn;
    fn generate_task_number() -> int { 0 }
    // Generate some state locally
    let child_task_number = generate_task_number();
    
    spawn(proc() {
        // Capture it in the remote task
        println!("I am child number {}", child_task_number);
    });
}