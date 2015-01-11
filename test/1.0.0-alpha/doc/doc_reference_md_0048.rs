fn main() {
    const BIT1: uint = 1 << 0;

    const BIT2: uint = 1 << 1;

    

    const BITS: [uint; 2] = [BIT1, BIT2];

    const STRING: &'static str = "bitstring";

    

    struct BitsNStrings<'a> {

        mybits: [uint; 2],

        mystring: &'a str

    }

    

    const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {

        mybits: BITS,

        mystring: STRING

    };

}
