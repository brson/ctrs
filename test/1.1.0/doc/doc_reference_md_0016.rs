fn main() { }

mod quux {

    pub use quux::foo::{bar, baz};



    pub mod foo {

        pub fn bar() { }

        pub fn baz() { }

    }

}

