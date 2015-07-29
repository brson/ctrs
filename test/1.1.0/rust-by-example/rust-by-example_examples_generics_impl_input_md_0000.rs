fn main() {
    struct S; // A null struct

    struct GenericVal<T>(T,);

    

    // impl of GenericVal we specifically specialize:

    impl GenericVal<f32> {} // Specialize to `f32`

    impl GenericVal<S> {} // Specialize to `S` defined above

    

    // `<T>` Must precede the type to remain generic

    impl <T> GenericVal<T> {}

}
