fn main() {
    use std::fmt;
    use std::io::{self, Write};
    
    fmt::format(format_args!("this returns {}", "String"));
    
    let mut some_writer = io::stdout();
    write!(&mut some_writer, "{}", format_args!("print with a {}", "macro"));
    
    fn my_fmt_fn(args: fmt::Arguments) {
    write!(&mut io::stdout(), "{}", args);
    }
    my_fmt_fn(format_args!("or a {} too", "function"));
}
