use farm::chicken;
use farm::cow;
use farm::barn;

mod farm {
    pub fn chicken() { println!("cluck cluck"); }
    pub fn cow() { println!("mooo"); }

    pub mod barn {
        pub fn hay() { println!("..."); }
    }
}

fn main() {
    println!("Hello farm!");

    // Can now refer to those names directly:
    chicken();
    cow();
    barn::hay();
}
