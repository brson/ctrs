fn main() {
    use std::fmt::Display;        trait Error: Display {        fn description(&self) -> &str;            fn cause(&self) -> Option<&Error> { None }    }}
