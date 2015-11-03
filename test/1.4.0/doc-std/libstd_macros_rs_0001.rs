fn main() {
    use std::io::{self, Write};
    
    print!("this ");
    print!("will ");
    print!("be ");
    print!("on ");
    print!("the ");
    print!("same ");
    print!("line ");
    
    io::stdout().flush().unwrap();
    
    print!("this string has a newline, why not choose println! instead?\n");
    
    io::stdout().flush().unwrap();
}
