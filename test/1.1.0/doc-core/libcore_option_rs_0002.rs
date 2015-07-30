fn main() {
    let msg = Some("howdy");
    
    // Take a reference to the contained string
    match msg {
    Some(ref m) => println!("{}", *m),
    None => ()
    }
    
    // Remove the contained string, destroying the Option
    let unwrapped_msg = match msg {
    Some(m) => m,
    None => "default message"
    };
}
