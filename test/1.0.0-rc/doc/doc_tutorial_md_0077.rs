fn main() {
    let mut x = 5i;
    {
        let y = &x; // `x` is now frozen. It cannot be modified or re-assigned.
    }
    // `x` is now unfrozen again
    x = 3;
}