fn main() {
    static BIT1: uint = 1 << 0;
    static BIT2: uint = 1 << 1;
    
    static BITS: [uint, ..2] = [BIT1, BIT2];
    static STRING: &'static str = "bitstring";
    
    struct BitsNStrings<'a> {
        mybits: [uint, ..2],
        mystring: &'a str
    }
    
    static bits_n_strings: BitsNStrings<'static> = BitsNStrings {
        mybits: BITS,
        mystring: STRING
    };
}