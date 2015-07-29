use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // Spawn the `wc` command
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
        Ok(process) => process,
    };

    {
        // Write a string to the `stdin` of `wc`.
        //
        // `stdin` has type `Option<ChildStdin>`, but since we know this instance
        // must have one, we can directly `unwrap` it.
        match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}",
                               Error::description(&why)),
            Ok(_) => println!("sent pangram to wc"),
        }

        // `stdin` gets `drop`ed here, and the pipe is closed.
        //
        // This is very important, otherwise `wc` wouldn't start processing the
        // input we just sent.
    }

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           Error::description(&why)),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
