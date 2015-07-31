fn main() {
    use std::io;
    use std::fs::File;
    use std::io::prelude::*;
    
    fn write_to_file_using_try() -> Result<(), io::Error> {
    let mut file = try!(File::create("my_best_friends.txt"));
    try!(file.write_all(b"This is a list of my best friends."));
    println!("I wrote to the file");
    Ok(())
    }
    // This is equivalent to:
    fn write_to_file_using_match() -> Result<(), io::Error> {
    let mut file = try!(File::create("my_best_friends.txt"));
    match file.write_all(b"This is a list of my best friends.") {
    Ok(_) => (),
    Err(e) => return Err(e),
    }
    println!("I wrote to the file");
    Ok(())
    }
}
