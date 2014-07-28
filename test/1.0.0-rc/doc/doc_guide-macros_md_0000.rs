fn main() {
    enum T { SpecialA(uint), SpecialB(uint) }
    fn f() -> uint {
    let input_1 = SpecialA(0);
    let input_2 = SpecialA(0);
    match input_1 {
        SpecialA(x) => { return x; }
        _ => {}
    }
    // ...
    match input_2 {
        SpecialB(x) => { return x; }
        _ => {}
    }
    return 0u;
    }
}