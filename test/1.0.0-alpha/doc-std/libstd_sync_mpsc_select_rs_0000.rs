fn main() {
    use std::sync::mpsc::channel;
    
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    
    tx1.send(1i).unwrap();
    tx2.send(2i).unwrap();
    
    select! {
        val = rx1.recv() => {
            assert_eq!(val.unwrap(), 1i);
        },
        val = rx2.recv() => {
            assert_eq!(val.unwrap(), 2i);
        }
    }
}
