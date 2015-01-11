pub fn increment(x: uint) -> uint {

    x + 1

}



#[macro_export]

macro_rules! inc_a {

    ($x:expr) => ( ::increment($x) )

}



#[macro_export]

macro_rules! inc_b {

    ($x:expr) => ( ::mylib::increment($x) )

}

fn main() { }

