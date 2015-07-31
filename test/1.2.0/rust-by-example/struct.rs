// First attempt: No explicit lifetimes
// Error! Compiler needs explicit lifetime
//struct Singleton {
    //one: &mut i32,
//}
// TODO ^ Try uncommenting this struct

// Second attempt: Add lifetimes to all the references
struct Pair<'a, 'b> {
    one: &'a mut i32,
    two: &'b mut i32,
}

fn main() {
    // Let us say that `one` has lifetime `o`
    let mut one = 1;

    {
        // And that `two` has lifetime `t`
        // `two` has a shorter (and different) lifetime than `one` (`'t < 'o`)
        let mut two = 2;

        println!("Before: ({}, {})", one, two);

        // `Pair` gets specialized for `'a = 'o` and `'b = 't`
        let pair = Pair { one: &mut one, two: &mut two };

        *pair.one = 2;
        *pair.two = 1;

        println!("After: ({}, {})", pair.one, pair.two);
    }
}
