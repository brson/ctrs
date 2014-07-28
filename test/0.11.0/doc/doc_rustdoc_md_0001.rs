#[doc = "
Calculates the factorial of a number.

Given the input integer `n`, this function will calculate `n!` and return it.
"]
pub fn factorial(n: int) -> int { if n < 2 {1} else {n * factorial(n - 1)} }
fn main() {}
