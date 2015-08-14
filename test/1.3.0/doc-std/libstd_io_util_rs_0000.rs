fn main() {
    use std::io;
    
    fn foo() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];
    
    try!(io::copy(&mut reader, &mut writer));
    
    assert_eq!(reader, &writer[..]);
    Ok(())
    }
}
