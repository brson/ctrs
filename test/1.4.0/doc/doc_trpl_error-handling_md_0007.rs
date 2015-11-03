fn main() {
    fn find(_: &str, _: char) -> Option<usize> { None }
    // Returns the extension of the given file name, where the extension is defined
    // as all characters proceding the first `.`.
    // If `file_name` has no `.`, then `None` is returned.
    fn extension_explicit(file_name: &str) -> Option<&str> {
        match find(file_name, '.') {
            None => None,
            Some(i) => Some(&file_name[i+1..]),
        }
    }
}
