fn main() {
    let mut owned = box 10i;
    
    let mut value = 20i;
    let borrowed = &mut value;
    
    *owned = *borrowed + 100;
    *borrowed = *owned + 1000;
}