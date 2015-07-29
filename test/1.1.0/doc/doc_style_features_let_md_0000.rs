fn main() {
    fn use_mutex(m: sync::mutex::Mutex<int>) {
        let guard = m.lock();
        do_work(guard);
        drop(guard); // unlock the lock
        // do other work
    }
}
