fn add_one(x: &int) -> int {
    *x + 1
}

fn main() {
    let x = box 5i;

    println!("{}", add_one(&*x));
}
