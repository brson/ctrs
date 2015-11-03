fn main() {
    fn find(_: &str, _: char) -> Option<usize> { None }
    // Returns the extension of the given file name, where the extension is defined
    // as all characters proceding the first `.`.
    // If `file_name` has no `.`, then `None` is returned.
    fn extension(file_name: &str) -> Option<&str> {
        find(file_name, '.').map(|i| &file_name[i+1..])
    }
}
