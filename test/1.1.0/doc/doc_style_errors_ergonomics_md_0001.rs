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
        match file.write_line(&format!("name: {}", info.name)) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match file.write_line(&format!("age: {}", info.age)) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        return file.write_line(&format!("rating: {}", info.rating));
    }
}
