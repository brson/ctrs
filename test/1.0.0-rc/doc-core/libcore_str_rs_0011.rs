fn main() {
    // composed forms of `ö` and `é`
    let c = "Löwe 老虎 Léopard"; // German, Simplified Chinese, French
    // decomposed forms of `ö` and `é`
    let d = "Lo\u0308we 老虎 Le\u0301opard";
    
    assert_eq!(c.char_len(), 15);
    assert_eq!(d.char_len(), 17);
    
    assert_eq!(c.len(), 21);
    assert_eq!(d.len(), 23);
    
    // the two strings *look* the same
    println!("{}", c);
    println!("{}", d);
}
