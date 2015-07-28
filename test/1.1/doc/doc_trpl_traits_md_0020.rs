fn main() {
    trait Foo {

    fn bar(&self);

    fn baz(&self) { println!("We called baz."); }

    }

    struct UseDefault;

    

    impl Foo for UseDefault {

        fn bar(&self) { println!("We called bar."); }

    }

    

    struct OverrideDefault;

    

    impl Foo for OverrideDefault {

        fn bar(&self) { println!("We called bar."); }

    

        fn baz(&self) { println!("Override baz!"); }

    }

    

    let default = UseDefault;

    default.baz(); // prints "We called baz."

    

    let over = OverrideDefault;

    over.baz(); // prints "Override baz!"

}
