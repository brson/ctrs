fn main() {
    fn use_mutex(m: sync::mutex::Mutex<int>) {
        do_work(m.lock());
        // do other work
    }
}
