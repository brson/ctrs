use std::thread;
use std::sync::mpsc;

fn main() {
let (tx, rx) = mpsc::channel();

for _ in 0..10 {
let tx = tx.clone();

thread::spawn(move || {
let answer = 42u32;

tx.send(answer);
});
}

rx.recv().ok().expect("Could not receive answer");
}
