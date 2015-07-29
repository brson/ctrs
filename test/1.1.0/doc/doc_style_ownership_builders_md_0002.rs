fn main() {
    // A simplified excerpt from std::thread::Builder
    
    impl ThreadBuilder {
    Name the thread-to-be. Currently the name is used for identification    only in failure messages.        pub fn named(mut self, name: String) -> ThreadBuilder {
            self.name = Some(name);
            self
        }
    
    Redirect thread-local stdout.        pub fn stdout(mut self, stdout: Box<Writer + Send>) -> ThreadBuilder {
            self.stdout = Some(stdout);
    //   ^~~~~~ this is owned and cannot be cloned/re-used            self
        }
    
    Creates and executes a new child thread.        pub fn spawn(self, f: proc():Send) {
            // consume self
            ...
        }
    }
}
