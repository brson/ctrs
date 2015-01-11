fn main() {
    enum T { SpecialA(uint), SpecialB(uint) }

    fn f() -> uint {

    let input_1 = T::SpecialA(0);

    let input_2 = T::SpecialA(0);

    match input_1 {

        T::SpecialA(x) => { return x; }

        _ => {}

    }

    // ...

    match input_2 {

        T::SpecialB(x) => { return x; }

        _ => {}

    }

    return 0u;

    }

}
