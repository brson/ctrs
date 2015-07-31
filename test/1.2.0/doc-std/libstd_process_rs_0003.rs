fn main() {
    use std::process::Command;
    
    let status = Command::new("ls").status().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
    });
    
    println!("process exited with: {}", status);
}
