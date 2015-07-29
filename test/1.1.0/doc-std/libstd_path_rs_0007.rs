fn main() {
    use std::path::Path;        let path = Path::new("/tmp/foo/bar.txt");    let file = path.file_name();    let extension = path.extension();    let parent_dir = path.parent();}
