// `farm.rs` - contains body of module 'farm' in the crate root
pub fn chicken() { println!("cluck cluck"); }
pub fn cow() { println!("mooo"); }

pub mod barn {
    pub fn hay() { println!("..."); }
}
fn main() { }
