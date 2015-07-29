fn main() {
    fn foo_bar(a: Bar, b: Bar,
               c: |Bar|) -> Bar {
        ...
    }
    
    // Same line is fine:
    foo_bar(x, y, |z| { z.transpose(y) });
    
    // Indented body on new line is also fine:
    foo_bar(x, y, |z| {
        z.quux();
        z.rotate(x)
    })
}
