fn main() {
    use std::sync::mpsc::sync_channel;
    use std::thread;
    
    let (tx, rx) = sync_channel(1);
    
    // this returns immediately
    tx.send(1).unwrap();
    
    thread::spawn(move|| {
    // this will block until the previous message has been received
    tx.send(2).unwrap();
    });
    
    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);
}
