fn main() {
    use std::process::Command;    let output = Command::new("cat").arg("foo.txt").output().unwrap_or_else(|e| {        panic!("failed to execute process: {}", e)    });        println!("status: {}", output.status);    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));}
