fn main() {
    use std::sync::mpsc::channel;
    
    let (tx, rx) = channel();
    
    // This send is always successful
    tx.send(1).unwrap();
    
    // This send will fail because the receiver is gone
    drop(rx);
    assert_eq!(tx.send(1).err().unwrap().0, 1);
}
