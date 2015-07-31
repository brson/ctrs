fn main() {
    use std::io;
    use std::fs::{self, DirEntry};
    use std::path::Path;
    
    // one possible implementation of fs::walk_dir only visiting files
    fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
        if try!(fs::metadata(dir)).is_dir() {
            for entry in try!(fs::read_dir(dir)) {
                let entry = try!(entry);
                if try!(fs::metadata(entry.path())).is_dir() {
                    try!(visit_dirs(&entry.path(), cb));
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}
