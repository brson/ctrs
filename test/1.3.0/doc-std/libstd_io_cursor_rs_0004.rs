fn main() {
    use std::io::Cursor;
    
    let mut buff = Cursor::new(Vec::new());
    fn force_inference(_: &Cursor<Vec<u8>>) {}
    force_inference(&buff);
    
    let reference = buff.get_mut();
}
