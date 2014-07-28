bitflags!(
    flags Flags: u32 {
        static FlagA       = 0x00000001,
        static FlagB       = 0x00000010,
        static FlagC       = 0x00000100,
        static FlagABC     = FlagA.bits
                           | FlagB.bits
                           | FlagC.bits
    }
)

fn main() {
    let e1 = FlagA | FlagC;
    let e2 = FlagB | FlagC;
    assert!((e1 | e2) == FlagABC);   // union
    assert!((e1 & e2) == FlagC);     // intersection
    assert!((e1 - e2) == FlagA);     // set difference
    assert!(!e2 == FlagA);           // set complement
}
