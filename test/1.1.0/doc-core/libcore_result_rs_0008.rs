fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    
    struct Info {
        name: String,
        age: i32,
        rating: i32,
    }
    
    fn write_info(info: &Info) -> io::Result<()> {
        let mut file = try!(File::create("my_best_friends.txt"));
        // Early return on error
        if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
            return Err(e)
        }
        if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
            return Err(e)
        }
        if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
            return Err(e)
        }
        Ok(())
    }
}
