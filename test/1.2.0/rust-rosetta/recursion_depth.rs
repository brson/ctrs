// Implements http://rosettacode.org/wiki/Find_limit_of_recursion
#[allow(unconditional_recursion)]

fn recursion(n: i32) {
    println!("deep: {}", n);
    recursion(n + 1);
}

fn main() {
    recursion(0);
}
