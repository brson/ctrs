fn main() {
    // Searches `haystack` for the Unicode character `needle`. If one is found, the
    // byte offset of the character is returned. Otherwise, `None` is returned.
    fn find(haystack: &str, needle: char) -> Option<usize> {
        for (offset, c) in haystack.char_indices() {
            if c == needle {
                return Some(offset);
            }
        }
        None
    }
}
