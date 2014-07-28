fn main() {
    let x = &mut 5i;

    if *x < 10 {
        let y = &x;

        println!("Oh no: {}", y);
        return;
    }

    *x -= 1;

    println!("Oh no: {}", x);
}
