fn main() {
    use std::sync::mpsc::channel;
    use std::thread;
    
    // tx is is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    let (tx, rx) = channel();
    
    // Spawn off an expensive computation
    thread::spawn(move|| {
      fn expensive_computation() {}
        tx.send(expensive_computation()).unwrap();
    });
    
    // Do some useful work for awhile
    
    // Let's see what that answer was
    println!("{:?}", rx.recv().unwrap());
}
