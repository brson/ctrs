pub fn increment(x: u32) -> u32 {

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

