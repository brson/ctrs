fn main() {
    /// `hello` is a function that prints a greeting that is personalized based on
    /// the name given.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the person you'd like to greet.
    ///
    /// # Example
    ///
    /// ```rust
    /// let name = "Steve";
    /// hello(name); // prints "Hello, Steve!"
    /// ```
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
}