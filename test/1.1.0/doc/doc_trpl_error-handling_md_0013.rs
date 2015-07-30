fn main() {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    
    struct Info {
    name: String,
    age: i32,
    rating: i32,
    }
    
    fn write_info(info: &Info) -> io::Result<()> {
    let mut file = File::create("my_best_friends.txt").unwrap();
    
    if let Err(e) = writeln!(&mut file, "name: {}", info.name) {
    return Err(e)
    }
    if let Err(e) = writeln!(&mut file, "age: {}", info.age) {
    return Err(e)
    }
    if let Err(e) = writeln!(&mut file, "rating: {}", info.rating) {
    return Err(e)
    }
    
    return Ok(());
    }
}
