fn main() {
    // A fixed-size vector
    let numbers = [1i, 2, 3];
    let more_numbers = numbers;
    
    // The type of a fixed-size vector is written as `[Type, ..length]`
    let five_zeroes: [int, ..5] = [0, ..5];
}