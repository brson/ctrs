fn main() {
    use std::path::Path;        let s = String::from("bar.txt");    let p = Path::new(&s);    Path::new(&p);}
