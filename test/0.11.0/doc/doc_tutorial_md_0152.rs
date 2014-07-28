use farm::{chicken, cow};
mod farm {
    pub fn cow() { println!("Did I already mention how hidden and ninja I am?") }
    pub fn chicken() { println!("I'm Bat-chicken, guardian of the hidden tutorial code.") }
}
fn main() { cow(); chicken() }
