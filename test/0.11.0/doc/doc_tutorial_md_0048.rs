fn main() {
    {
        // an integer allocated on the heap
        let y = box 10i;
    }
    // the destructor frees the heap memory as soon as `y` goes out of scope
}