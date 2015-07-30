fn main() {
    use std::borrow::Cow;
    
    fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
    let v = input[i];
    if v < 0 {
    // clones into a vector the first time (if not already owned)
    input.to_mut()[i] = -v;
    }
    }
    }
}
