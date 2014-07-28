fn main() {
    use std::default::Default;
    
    let i: i8 = Default::default();
    let (x, y): (Option<String>, f64) = Default::default();
    let (a, b, (c, d)): (int, uint, (bool, bool)) = Default::default();
}
