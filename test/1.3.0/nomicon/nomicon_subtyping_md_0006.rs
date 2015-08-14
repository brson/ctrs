fn main() {
    use std::cell::Cell;
    
    struct Foo<'a, 'b, A: 'a, B: 'b, C, D, E, F, G, H> {
        a: &'a A,     // variant over 'a and A
        b: &'b mut B, // invariant over 'b and B
        c: *const C,  // variant over C
        d: *mut D,    // invariant over D
        e: Vec<E>,    // variant over E
        f: Cell<F>,   // invariant over F
        g: G,         // variant over G
        h1: H,        // would also be variant over H except...
        h2: Cell<H>,  // invariant over H, because invariance wins
    }
}
