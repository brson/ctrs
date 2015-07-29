fn main() {
    // One-liners
    ThreadBuilder::new().named("my_thread").spawn(proc() { ... });
    
    // Complex configuration
    let mut thread = ThreadBuilder::new();
    thread = thread.named("my_thread_2"); // must re-assign to retain ownership
    
    if reroute {
        thread = thread.stdout(mywriter);
    }
    
    thread.spawn(proc() { ... });
}
