fn main() {
    // One-liners
    Command::new("/bin/cat").arg("file.txt").spawn();
    
    // Complex configuration
    let mut cmd = Command::new("/bin/ls");
    cmd.arg(".");
    
    if size_sorted {
        cmd.arg("-S");
    }
    
    cmd.spawn();
}
