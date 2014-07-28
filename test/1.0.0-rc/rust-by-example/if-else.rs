fn main() {
    let n = 5i;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `int`
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // This expression must return an `int` as well
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon
        };

    println!("{} -> {}", n, big_n);
}
