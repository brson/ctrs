fn succ(x: &int) -> int { *x + 1 }

fn main() {

    let x = 5i;
    let y = &x;

    println!("{}", succ(y));
    println!("{}", succ(&x));
}
