fn main() {
    use std::process::Command;
    
    let output = Command::new("sh")
    .arg("-c")
    .arg("echo hello")
    .output()
    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let hello = output.stdout;
}
