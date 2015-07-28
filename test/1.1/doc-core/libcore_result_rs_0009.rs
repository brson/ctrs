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
        try!(file.write_all(format!("name: {}\n", info.name).as_bytes()));
        try!(file.write_all(format!("age: {}\n", info.age).as_bytes()));
        try!(file.write_all(format!("rating: {}\n", info.rating).as_bytes()));
        Ok(())
    }
}
