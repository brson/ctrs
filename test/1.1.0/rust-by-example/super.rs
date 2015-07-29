fn function() {
    println!("called `function()`");
}

mod my {
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope
        print!("called `my::indirect_call()`, that\n> ");

        // `my::function` can be called directly
        function();

        {
            // This will bind to the `cool::function` in the *crate* scope
            // In this case the crate scope is the outermost scope
            use cool::function as root_cool_function;

            print!("> ");
            root_cool_function();
        }

        {
            // `self` refers to the current module scope, in this case: `my`
            use self::cool::function as my_cool_function;

            print!("> ");
            my_cool_function();
        }

        {
            // `super` refers to the parent scope, i.e. outside of the `my`
            // module
            use super::function as root_function;

            print!("> ");
            root_function();
        }
    }

    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

fn main() {
    my::indirect_call();
}
