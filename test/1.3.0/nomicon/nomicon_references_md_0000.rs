fn main() {
    let x = &mut (1, 2);
    {
        // reborrow x to a subfield
        let y = &mut x.0;
        // y is now live, but x isn't
        *y = 3;
    }
    // y goes out of scope, so x is live again
    *x = (5, 7);
}
