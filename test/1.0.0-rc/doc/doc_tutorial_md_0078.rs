fn main() {
    let owned = box 10i;
    let borrowed = &20i;
    
    let sum = *owned + *borrowed;
}