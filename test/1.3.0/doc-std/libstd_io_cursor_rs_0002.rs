fn main() {
    use std::io::Cursor;
    
    let buff = Cursor::new(Vec::new());
    fn force_inference(_: &Cursor<Vec<u8>>) {}
    force_inference(&buff);
    
    let vec = buff.into_inner();
}
