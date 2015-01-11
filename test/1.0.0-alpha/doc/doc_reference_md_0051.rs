fn main() {
    fn atomic_add(_: &mut uint, _: uint) -> uint { 2 }

    

    static mut LEVELS: uint = 0;

    

    // This violates the idea of no shared state, and this doesn't internally

    // protect against races, so this function is `unsafe`

    unsafe fn bump_levels_unsafe1() -> uint {

        let ret = LEVELS;

        LEVELS += 1;

        return ret;

    }

    

    // Assuming that we have an atomic_add function which returns the old value,

    // this function is "safe" but the meaning of the return value may not be what

    // callers expect, so it's still marked as `unsafe`

    unsafe fn bump_levels_unsafe2() -> uint {

        return atomic_add(&mut LEVELS, 1);

    }

}
