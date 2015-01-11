enum T { SpecialA(uint),SpecialB(uint),SpecialC(uint),SpecialD(uint)}

fn f() -> uint {

let input_1 = T::SpecialA(0);

let input_2 = T::SpecialA(0);

macro_rules! early_return {

    ($inp:expr, [ $($sp:path),+ ]) => (

        match $inp {

            $(

                $sp(x) => { return x; }

            )+

            _ => {}

        }

    )

}

// ...

early_return!(input_1, [T::SpecialA,T::SpecialC,T::SpecialD]);

// ...

early_return!(input_2, [T::SpecialB]);

return 0;

}

fn main() {}

