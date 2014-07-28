fn main() {
    fn foo() -> (u64, u64, u64, u64, u64, u64) {
        (5, 5, 5, 5, 5, 5)
    }
    
    let x = box foo(); // allocates a box, and writes the integers directly to it
}