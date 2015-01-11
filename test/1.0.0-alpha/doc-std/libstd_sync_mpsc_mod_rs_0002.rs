fn main() {
    use std::sync::mpsc::channel;
    
    // The call to recv() will return an error because the channel has already
    // hung up (or been deallocated)
    let (tx, rx) = channel::<int>();
    drop(tx);
    assert!(rx.recv().is_err());
}
