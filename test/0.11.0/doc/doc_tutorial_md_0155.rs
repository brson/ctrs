mod farm {
    pub use self::barn::hay;

    pub fn chicken() { println!("cluck cluck"); }
    pub fn cow() { println!("mooo"); }

    mod barn {
        pub fn hay() { println!("..."); }
    }
}

fn main() {
    farm::chicken();
    farm::cow();
    farm::hay();
}
