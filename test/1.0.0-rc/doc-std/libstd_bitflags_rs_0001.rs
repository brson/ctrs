use std::fmt;

bitflags!(
    flags Flags: u32 {
        static FlagA   = 0x00000001,
        static FlagB   = 0x00000010
    }
)

impl Flags {
    pub fn clear(&mut self) {
        self.bits = 0;  // The `bits` field can be accessed from within the
                        // same module where the `bitflags!` macro was invoked.
    }
}

impl fmt::Show for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi!")
    }
}

fn main() {
    let mut flags = FlagA | FlagB;
    flags.clear();
    assert!(flags.is_empty());
    assert_eq!(format!("{}", flags).as_slice(), "hi!");
}
