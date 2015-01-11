macro_rules! loop_x {

    ($e: expr) => (

        // $e will not interact with this 'x

        'x: loop {

            println!("Hello!");

            $e

        }

    );

}



fn main() {

    'x: loop {

        loop_x!(break 'x);

        println!("I am never printed.");

    }

}

