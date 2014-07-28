fn main() {
    use std::str;
    use std::str::{ScalarValue, LoneSurrogate};
    
    // 𝄞mus<invalid>ic<invalid>
    let v = [0xD834, 0xDD1E, 0x006d, 0x0075,
             0x0073, 0xDD1E, 0x0069, 0x0063,
             0xD834];
    
    assert_eq!(str::utf16_items(v).collect::<Vec<_>>(),
               vec![ScalarValue('𝄞'),
                    ScalarValue('m'), ScalarValue('u'), ScalarValue('s'),
                    LoneSurrogate(0xDD1E),
                    ScalarValue('i'), ScalarValue('c'),
                    LoneSurrogate(0xD834)]);
}
