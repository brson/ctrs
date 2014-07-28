// This is the crate root

mod farm {
    // This is the body of module 'farm' declared in the crate root.

    fn chicken() { println!("cluck cluck"); }
    fn cow() { println!("mooo"); }

    mod barn {
        // Body of module 'barn'

        fn hay() { println!("..."); }
    }
}

fn main() {
    println!("Hello farm!");
}
