fn main() {
    use std::io::{File, Open, Write, IoError};
    
    struct Info {
        name: String,
        age: int,
        rating: int
    }
    
    fn write_info(info: &Info) -> Result<(), IoError> {
        let mut file = File::open_mode(&Path::new("my_best_friends.txt"),
                                       Open, Write);
        // Early return on error
        try!(file.write_line(&format!("name: {}", info.name)));
        try!(file.write_line(&format!("age: {}", info.age)));
        try!(file.write_line(&format!("rating: {}", info.rating)));
        return Ok(());
    }
}
