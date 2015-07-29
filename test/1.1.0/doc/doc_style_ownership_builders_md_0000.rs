fn main() {
    // NOTE: the actual Command API does not use owned Strings;
    // this is a simplified version.
    
    pub struct Command {
        program: String,
        args: Vec<String>,
        cwd: Option<String>,
        // etc
    }
    
    impl Command {
        pub fn new(program: String) -> Command {
            Command {
                program: program,
                args: Vec::new(),
                cwd: None,
            }
        }
    
    Add an argument to pass to the program.        pub fn arg<'a>(&'a mut self, arg: String) -> &'a mut Command {
            self.args.push(arg);
            self
        }
    
    Add multiple arguments to pass to the program.        pub fn args<'a>(&'a mut self, args: &[String])
                        -> &'a mut Command {
            self.args.push_all(args);
            self
        }
    
    Set the working directory for the child process.        pub fn cwd<'a>(&'a mut self, dir: String) -> &'a mut Command {
            self.cwd = Some(dir);
            self
        }
    
    Executes the command as a child process, which is returned.        pub fn spawn(&self) -> IoResult<Process> {
            ...
        }
    }
}
