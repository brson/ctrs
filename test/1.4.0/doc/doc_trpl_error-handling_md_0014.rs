fn main() {
    fn extension(file_name: &str) -> Option<&str> { None }
    fn file_name(file_path: &str) -> Option<&str> { None }
    fn file_path_ext(file_path: &str) -> Option<&str> {
        file_name(file_path).and_then(extension)
    }
}
