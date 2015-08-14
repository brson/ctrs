fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::LineWriter;
    
    fn foo() -> std::io::Result<()> {
    let road_not_taken = b"I shall be telling this with a sigh
    Somewhere ages and ages hence:
    Two roads diverged in a wood, and I -
    I took the one less traveled by,
    And that has made all the difference.";
    
    let file = try!(File::create("poem.txt"));
    let mut file = LineWriter::new(file);
    
    for &byte in road_not_taken.iter() {
       file.write(&[byte]).unwrap();
    }
    
    // let's check we did the right thing.
    let mut file = try!(File::open("poem.txt"));
    let mut contents = String::new();
    
    try!(file.read_to_string(&mut contents));
    
    assert_eq!(contents.as_bytes(), &road_not_taken[..]);
    Ok(())
    }
}
