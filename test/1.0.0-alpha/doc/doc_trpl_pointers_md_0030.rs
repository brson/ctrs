fn main() {
    use std::boxed::Box;

    {

        let x = Box::new(5i);

    

        // stuff happens

    

    } // x is destructed and its memory is free'd here

}
