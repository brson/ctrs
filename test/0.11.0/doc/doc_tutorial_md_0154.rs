use egg_layer = farm::chicken;
mod farm { pub fn chicken() { println!("Laying eggs is fun!")  } }
// ...

fn main() {
    egg_layer();
}
